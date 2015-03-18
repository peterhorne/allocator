use std::collections::HashMap;

pub type Uuid = String;
pub type Quantity = i32;
pub type ResourceMap = HashMap<Uuid, Quantity>;
pub type ConsumerMap = HashMap<Uuid, ResourceMap>;

trait Command {
    pub fn new_from_string(args: &str) -> Self;
    pub fn process(&self, &resources: ResourceMap, &consumers: ConsumerMap) -> Result<Self, &str>;
}

pub struct Resource {
    pub id: Uuid,
    pub quantity: Quantity,
}

impl Command for Resource {
    pub fn new(id: Uuid, quantity: Quantity) -> Resource {
        Resource { id: id, quantity: quantity }
    }

    // args: "<id> <quantity>"
    // args: "aaaa 23"
    pub fn new_from_string(args: &str) -> Result<Resource, &str> {
        match &args.split(' ').collect::<Vec<&str>>()[..] {
            [id, quantity] => {
                match quantity.parse::<i32>() {
                    Ok(quantity) => Ok(Resource::new(id.to_string(), quantity)),
                    Err(_) => Err("Quantity is not i32."),
                }
            },
            _ => Err("Invalid arguments."),
        }
    }
}

struct Resource {
    id: Uuid,
    amount: u32,
}

impl Resource {
    pub fn apply() {

    }
}

type Resources = HashMap<Uuid, Resource>;

impl Resources {
    pub fn add(&mut self, others: Resources) -> Resources {
        for other in others {
            match self.entry(other.id) {
                Vacant(entry)
            }
        }
    }

    pub fn subtract(&mut self, others: Resources) -> Resources {

    }
}

pub struct Consumer {
    pub id: Uuid,
    pub resources: ResourceMap,
}

impl Consumer {
    pub fn apply(&self, resources, consumers) -> Self {
        let entry = match consumers.entry(self.id) {
            Vacant(entry)   => entry
            Occupied(entry) => {
                resources.add(entry.get().resources);
                entry
            },
        };

        let results: ResourceMap = resources.subtract(self.resources);
        let consumer = Consumer::new(self.id, results);
        entry.set(consumer.clone());
        consumer
    }
}
