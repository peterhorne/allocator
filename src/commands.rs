use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp;

pub type Uuid = String;
pub type Quantity = u32;
pub type ResourceMap = HashMap<Uuid, Quantity>;
pub type ConsumerMap = HashMap<Uuid, ResourceMap>;

pub fn deserialise(args: &str) -> Result<Box<Command>, &'static str> {
    match args.trim().split(' ').collect::<Vec<&str>>().as_slice() {
        [name, args..] => match name {
            "RESOURCE" => Resource::deserialise(args),
            // "CONSUMER" => Consumer::parse(args),
            _ => Err("Unrecognised command."),
        },
        _ => Err("Unrecognised command."),
    }
}

pub trait Command {
    fn serialise(&self) -> String;
    fn process(&self, resources: &mut ResourceMap, consumers: &mut ConsumerMap) -> Box<Command>;
}

#[derive(Debug)]
pub struct Resource {
    pub id: Uuid,
    pub quantity: Quantity,
}

impl Resource {
    pub fn new(id: Uuid, quantity: Quantity) -> Resource {
        Resource { id: id, quantity: quantity }
    }

    pub fn deserialise(args: &[&str]) -> Result<Box<Command>, &'static str> {
        match args {
            [id, quantity] => {
                match quantity.parse::<u32>() {
                    Ok(quantity) => Ok(Box::new(Resource::new(id.to_string(), quantity))),
                    Err(_) => Err("Invalid arguments."),
                }
            },
            _ => Err("Invalid arguments."),
        }
    }
}

impl Command for Resource {
    fn serialise(&self) -> String {
        format!("RESOURCE {} {}", self.id, self.quantity)
    }

    fn process(&self, resources: &mut ResourceMap, consumers: &mut ConsumerMap) -> Box<Command> {
        let quantity = match resources.entry(self.id.clone()) {
            Occupied(mut entry) => {
                if self.quantity > *entry.get() {
                    entry.insert(self.quantity);
                    self.quantity
                } else {
                    *entry.get()
                }
            },
            Vacant(entry) => {
                *entry.insert(self.quantity)
            },
        };

        Box::new(Resource::new(self.id.clone(), self.quantity))
    }
}

// pub struct Consumer {
//     pub id: Uuid,
//     pub resources: ResourceMap,
// }

// impl Consumer {
//     pub fn apply(&self, resources, consumers) -> Self {
//         let entry = match consumers.entry(self.id) {
//             Vacant(entry)   => entry
//             Occupied(entry) => {
//                 resources.add(entry.get().resources);
//                 entry
//             },
//         };

//         let results: ResourceMap = resources.subtract(self.resources);
//         let consumer = Consumer::new(self.id, results);
//         entry.set(consumer.clone());
//         consumer
//     }
// }
