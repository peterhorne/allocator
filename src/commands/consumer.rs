use std::collections::HashMap;
use commands::{Uuid, Quantity, ResourceMap, ConsumerMap, Command};

#[derive(Debug)]
pub struct Consumer {
    pub id: Uuid,
    pub resources: ResourceMap,
}

impl Consumer {
    pub fn new(id: Uuid, resources: ResourceMap) -> Consumer {
        Consumer { id: id, resources: resources }
    }

    pub fn deserialise(args: &[&str]) -> Result<Box<Command>, &'static str> {
        match args {
            [id, resources..] => {
                let mut _resources: ResourceMap = HashMap::new();
                for resource in resources {
                    println!("{:?}", resource);
                    // match resource.split('=').collect::<Vec<&str>>() {
                    //     [resource_id, quantity] => { _resources.insert(resource_id, quantity) },
                    //     _  => { return Err("Invalid arguments.") },
                    // };
                };
                Ok(Box::new(Consumer::new(id.to_string(), _resources)))
            },
            _ => Err("Invalid arguments."),
        }
    }
}

impl Command for Consumer {
    fn serialise(&self) -> String {
        format!("CONSUME {}", self.id)
    }

    fn process(&self, resources: &mut ResourceMap, consumers: &mut ConsumerMap) -> Box<Command> {
        Box::new(Consumer::new(self.id.to_string(), HashMap::new()))
    }
}
