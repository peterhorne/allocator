use database::Database;

mod stock;
use self::stock::Stock;

mod consume;
use self::consume::Consume;

pub fn from_str(args: &str) -> Result<Box<Command>, &'static str> {
    match args.trim().split(' ').collect::<Vec<&str>>().as_ref() {
        [name, args..] => match name {
            "STOCK" =>   Stock::from_str(args),
            "CONSUME" => Consume::from_str(args),
            _ => Err("Unrecognised command."),
        },
        _ => Err("Unrecognised command."),
    }
}

pub trait Command {
    fn to_string(&self) -> String;
    fn execute(&self, database: &mut Database) -> Box<Command>;
}
