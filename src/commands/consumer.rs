use commands::Command;
// use commands::resource::Resource; // Remove Resource???
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
            // for resource in _resources {
            //     resources.push(try!(Resource::from_str(&[*resource])));
            // }
            Ok(Box::new(Consumer::new(id.to_string(), resources)))
        } else {
            Err("Invalid arguments.")
        }

        // match args {
        //     [id, _resources..] => {
        //         let mut resources: Vec<(String, u32)> = vec![];
        //         for resource in _resources {
        //             resources.push(try!(Resource::from_str(&[*resource])));
        //         }

        //         // let mut _resources: ResourceMap = HashMap::new();
        //         // for resource in resources {
        //         //     match resource.split('=').collect::<Vec<&str>>().as_slice() {
        //         //         [resource_id, quantity] => {
        //         //             match quantity.parse::<u32>() {
        //         //                 Ok(quantity) => { _resources.insert(resource_id.to_string(), quantity); },
        //         //                 Err(_) => { return Err("Invalid arguments.") },
        //         //             }
        //         //         },
        //         //         _  => { return Err("Invalid arguments.") },
        //         //     };
        //         // };
        //         Ok(Box::new(Consumer::new(id.to_string(), resources)))
        //     },
        //     _ => Err("Invalid arguments."),
        // }
    }
}

impl Command for Consumer {
    fn to_string(&self) -> String {
        format!("CONSUME {}", self.id)
    }

    fn execute(&self, database: &mut Database) -> Box<Command> {
        let result = database.consume(&self.id, &self.resources);
        // let resources = consumer.resources.iter()
        //     .map(|(stock_id, quantity)| format!("{}={}", stock_id, quantity))
        //     .collect::<Vec<String>>()
        //     .connect(" ");

        // let line = format!("{} {}\n", consumer.id, resources);
        Box::new(Consumer::new(self.id.to_string(), result))
    }
}
