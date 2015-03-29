use commands::Command;
use database::Database;

#[derive(Debug)]
pub struct Stock {
    pub id: String,
    pub quantity: u32,
}

impl Stock {
    pub fn new(id: String, quantity: u32) -> Stock {
        Stock { id: id, quantity: quantity }
    }

    pub fn from_str(args: &[&str]) -> Result<Box<Command>, &'static str> {
        if let [arg] = args {
            if let [id, quantity] = arg.split('=').collect::<Vec<&str>>().as_ref() {
                if let Ok(quantity) = quantity.parse::<u32>() {
                    return Ok(Box::new(Stock::new(id.to_string(), quantity)));
                };
            };
        };

        Err("Invalid arguments.")
    }
}

impl Command for Stock {
    fn to_string(&self) -> String {
        format!("STOCK {} {}", self.id, self.quantity)
    }

    fn execute(&self, database: &mut Database) -> Box<Command> {
        let result = database.stock(&self.id, self.quantity);
        Box::new(Stock::new(self.id.clone(), result))
    }
}
