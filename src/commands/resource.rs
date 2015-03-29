use commands::Command;
use database::Database;

#[derive(Debug)]
pub struct Resource {
    pub id: String,
    pub quantity: u32,
}

impl Resource {
    pub fn new(id: String, quantity: u32) -> Resource {
        Resource { id: id, quantity: quantity }
    }

    pub fn from_str(args: &[&str]) -> Result<Box<Command>, &'static str> {
        if let [arg] = args {
            if let [id, quantity] = arg.split('=').collect::<Vec<&str>>().as_ref() {
                if let Ok(quantity) = quantity.parse::<u32>() {
                    return Ok(Box::new(Resource::new(id.to_string(), quantity)));
                };
            };
        };

        Err("Invalid arguments.")

        // match args {
        //     [arg] => match arg.split('=').collect::<Vec<&str>>().as_slice() {
        //         [id, quantity] => {
        //             match quantity.parse::<u32>() {
        //                 Ok(quantity) => Ok(Box::new(Resource::new(id.to_string(), quantity))),
        //                 Err(_) => Err("Invalid arguments."),
        //             }
        //         },
        //         _ => Err("Invalid arguments."),
        //     },
        //     _ => Err("Invalid arguments."),
        // }
    }
}

impl Command for Resource {
    fn to_string(&self) -> String {
        format!("STOCK {} {}", self.id, self.quantity)
    }

    fn execute(&self, database: &mut Database) -> Box<Command> {
        let result = database.stock(&self.id, self.quantity);
        Box::new(Resource::new(self.id.clone(), result))
    }
}
