use commands::Command;
use database::Database;

#[derive(Debug)]
pub struct Consumer {
    pub id: String,
    pub resources: Vec<(String, u32)>,
}

impl Consumer {
    pub fn new(id: String, resources: Vec<(String, u32)>) -> Consumer {
        Consumer { id: id, resources: resources }
    }

    pub fn from_str(args: &[&str]) -> Result<Box<Command>, &'static str> {
        if let [id, _resources..] = args {
            let mut resources: Vec<(String, u32)> = vec![];
            for resource in _resources {
                if let [id, quantity] = resource.split('=').collect::<Vec<&str>>().as_ref() {
                    if let Ok(quantity) = quantity.parse::<u32>() {
                        resources.push((id.to_string(), quantity));
                    } else {
                        return Err("Invalid arguments");
                    }
                } else {
                    return Err("Invalid arguments");
                }
            }

            Ok(Box::new(Consumer::new(id.to_string(), resources)))
        } else {
            Err("Invalid arguments.")
        }
    }
}

impl Command for Consumer {
    fn to_string(&self) -> String {
        // let resources = consumer.resources.iter()
        //     .map(|(stock_id, quantity)| format!("{}={}", stock_id, quantity))
        //     .collect::<Vec<String>>()
        //     .connect(" ");

        // let line = format!("{} {}\n", consumer.id, resources);
        format!("CONSUME {}", self.id)
    }

    fn execute(&self, database: &mut Database) -> Box<Command> {
        let result = database.consume(&self.id, &self.resources);
        Box::new(Consumer::new(self.id.to_string(), result))
    }
}
