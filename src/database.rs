use std::collections::HashMap;

type ResourceMap = HashMap<Guid, Quantity>;
type Guid = String;
type Quantity = i32;

pub struct Database {
    resources: ResourceMap,
    consumers: HashMap<Guid, ResourceMap>, 
}

impl Database {
    pub fn new() -> Database {
        Database { resources: HashMap::new(), consumers: HashMap::new() }
    }
}
