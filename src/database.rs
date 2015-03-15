use std::collections::HashMap;
use consumer::Consumer;

pub type Uuid = String;
pub type Quantity = i32;
pub type ResourceMap = HashMap<Uuid, Quantity>;

pub enum Command {
    Resource,
    Consumer,
}

pub struct Database {
    resources: ResourceMap,
    consumers: HashMap<Uuid, ResourceMap>,
}

impl Database {
    pub fn new() -> Database {
        Database { resources: HashMap::new(), consumers: HashMap::new() }
    }

    pub fn apply(&mut self, command: Command) -> Command {
        command
    }

    pub fn stock(&mut self, resource_id: Uuid, quantity: i32) -> i32 {
        0
    }

    pub fn consume(&mut self, consumer: &Consumer) -> Consumer {
        Consumer::new("1234".to_string())
    }
}
