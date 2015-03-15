use std::collections::HashMap;
use database::{Uuid, Quantity, ResourceMap};

#[derive(Debug)]
pub struct Consumer {
    pub id: Uuid,
    pub resources: ResourceMap,
}

impl Consumer {
    pub fn new(id: Uuid) -> Consumer {
        Consumer { id: id, resources: ResourceMap::new() }
    }

    pub fn new_from_string(line: String) -> Option<Consumer> {
        let split = line.split(" ")
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();
        parse_request(split).ok()
    }
}

/// Parses cli args of the format: ./allocator 1234 aaaa=2 bbbb=2
/// 
/// Where:
///     1234 = Consumer ID
///     aaaa = Stock Item ID
fn parse_request<'a>(args: Vec<String>) -> Result<Consumer, String> {
    let consumer_id = match args.get(0) {
        Some(id) => id.clone(),
        None => { return Err("Missing consumer id.".to_string()) },
    };

    let mut resources = HashMap::new();
    for arg in &args[1..] {
        let mut split = arg.split("=");

        let stock_id = match split.next() {
            Some(id) => id.clone(),
            None => { return Err("Missing stock id.".to_string()) },
        };

        let quantity: Quantity = match split.next() {
            Some(num) => {
                match num.parse() {
                    Ok(num) => num,
                    Err(_) => { return Err(format!("Quantity is not an integer: {}", num)) },
                }
            },
            None => { return Err("Missing quantity".to_string()) },
        };

        resources.insert(stock_id.to_string(), quantity);
    }

    Ok(Consumer { id: consumer_id, resources: resources })
}
