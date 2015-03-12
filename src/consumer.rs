use std::collections::HashMap;

pub type AllocationMap = HashMap<String, Quantity>;
pub type Quantity = i32;

pub struct Consumer {
    pub id: String,
    pub allocations: AllocationMap,
}

/// Parses cli args of the format: ./allocator 1234 aaaa=2 bbbb=2
/// 
/// Where:
///     1234 = Consumer ID
///     aaaa = Stock Item ID
fn parse_request<'a>(args: Vec<String>) -> Result<Consumer, String> {
    let reservation_id = match args.get(0) {
        Some(id) => id.clone(),
        None => { return Err("Missing reservation id.".to_string()) },
    };

    let mut allocations = HashMap::new();
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

        allocations.insert(stock_id.to_string(), quantity);
    }

    Ok(Consumer { id: reservation_id, allocations: allocations })
}

impl Consumer {
    pub fn new_from_string(line: String) -> Option<Consumer> {
        let split = line.split(" ")
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();
        parse_request(split).ok()
    }
}
