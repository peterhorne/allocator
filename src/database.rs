use std::collections::HashMap;
use consumer::Consumer;

pub type Uuid = String;
pub type Quantity = i32;
pub type ResourceMap = HashMap<Uuid, Quantity>;
pub type ConsumerMap = HashMap<Uuid, ResourceMap>;

pub enum Command {
    Resource,
    Consumer,
}

pub struct Database {
    resources: ResourceMap,
    consumers: ConsumerMap,
}

impl Database {
    pub fn new() -> Database {
        Database { resources: HashMap::new(), consumers: HashMap::new() }
    }

    pub fn apply(&mut self, command: Command) -> Command {
        command
    }
}
