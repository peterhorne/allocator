use std::collections::HashMap;
use database::{Uuid, Quantity, ResourceMap};

#[derive(Debug)]
pub struct Resource {
    pub id: Uuid,
    pub quantity: Quantity,
}

impl Resource {
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
