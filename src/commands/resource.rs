use commands::{Uuid, Quantity, ResourceMap, ConsumerMap, Command};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp;

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
            [arg] => match arg.split('=').collect::<Vec<&str>>().as_slice() {
                [id, quantity] => {
                    match quantity.parse::<u32>() {
                        Ok(quantity) => Ok(Box::new(Resource::new(id.to_string(), quantity))),
                        Err(_) => Err("Invalid arguments."),
                    }
                },
                _ => Err("Invalid arguments."),
            },
            _ => Err("Invalid arguments."),
        }
    }
}

impl Command for Resource {
    fn serialise(&self) -> String {
        format!("STOCK {} {}", self.id, self.quantity)
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
