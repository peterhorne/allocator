use std::collections::HashMap;

pub type Uuid = String;
pub type Quantity = i32;
pub type ResourceMap = HashMap<Uuid, Quantity>;
pub type ConsumerMap = HashMap<Uuid, ResourceMap>;

pub fn parse(args: Vec<&str>) -> Result<Command, &str> {
    match args {
        [name, ..args] => match name {
            "RESOURCE" => Resource::parse(args),
            "CONSUMER" => Consumer::parse(args),
            _ => Err("Unrecognised command."),
        },
        _ => Err("Unrecognised command."),
    }
}

trait Command {
    pub fn parse(args: Vec<&str>) -> Result<Self, &str>;
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

    pub fn parse(args: Vec<&str>) -> Result<Self, &str> {
        match args {
            [id, quantity: u32] => Ok(Resource::new(id, quantity)),
            _ => Err("Invalid arguments."),
        }
    }

    pub fn process(&self, &resources, &consumers) -> Resource {
        let resource = match resources.entry(self.id) {
            Occupied(entry) => {
                entry.set(min(entry.get(), self.quantity));
                entry.clone()
            },
            Vacant(entry) => {
                entry.set(self);
                entry.clone()
            },
        }
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
