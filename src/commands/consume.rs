use commands::Command;
use database::Database;

#[derive(Debug)]
pub struct Consume {
    pub id: String,
    pub resources: Vec<(String, u32)>,
}

impl Consume {
    pub fn new(id: String, resources: Vec<(String, u32)>) -> Consume {
        Consume { id: id, resources: resources }
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

            Ok(Box::new(Consume::new(id.to_string(), resources)))
        } else {
            Err("Invalid arguments.")
        }
    }
}

impl Command for Consume {
    fn to_string(&self) -> String {
        let resources = self.resources.iter()
            .map(|&(ref stock_id, ref quantity)| format!("{}={}", stock_id, quantity))
            .collect::<Vec<String>>()
            .connect(" ");

        format!("CONSUME {} {}\n", self.id, resources)
    }

    fn execute(&self, database: &mut Database) -> Box<Command> {
        let result = database.consume(&self.id, &self.resources);
        Box::new(Consume::new(self.id.to_string(), result))
    }
}
