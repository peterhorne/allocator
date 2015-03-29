use database::Database;

mod resource;
use self::resource::Resource;

mod consumer;
use self::consumer::Consumer;

pub fn from_str(args: &str) -> Result<Box<Command>, &'static str> {
    match args.trim().split(' ').collect::<Vec<&str>>().as_ref() {
        [name, args..] => match name {
            "STOCK" =>   Resource::from_str(args),
            "CONSUME" => Consumer::from_str(args),
            _ => Err("Unrecognised command."),
        },
        _ => Err("Unrecognised command."),
    }
}

pub trait Command {
    fn to_string(&self) -> String;
    fn execute(&self, database: &mut Database) -> Box<Command>;
}
