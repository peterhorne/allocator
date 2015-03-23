use std::collections::HashMap;

mod resource;
use self::resource::Resource;

mod consumer;
use self::consumer::Consumer;

pub type Uuid = String;
pub type Quantity = u32;
pub type ResourceMap = HashMap<Uuid, Quantity>;
pub type ConsumerMap = HashMap<Uuid, ResourceMap>;

pub fn deserialise(args: &str) -> Result<Box<Command>, &'static str> {
    match args.trim().split(' ').collect::<Vec<&str>>().as_slice() {
        [name, args..] => match name {
            "RESOURCE" => Resource::deserialise(args),
            "CONSUMER" => Consumer::deserialise(args),
            _ => Err("Unrecognised command."),
        },
        _ => Err("Unrecognised command."),
    }
}

pub trait Command {
    fn serialise(&self) -> String;
    fn process(&self, resources: &mut ResourceMap, consumers: &mut ConsumerMap) -> Box<Command>;
}

