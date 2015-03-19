use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_io;
use std::old_io::stdio::{StdinReader};

type ItemMap = HashMap<&'static str, i32>;

fn diff(a: &ItemMap, b: &ItemMap) -> ItemMap {
    let mut result = b.clone();
    for (key, &value) in a {
        match result.entry(key) {
            Occupied(mut v) => { *v.get_mut() -= value; },
            Vacant(mut v)   => { v.insert(-value); },
        }
    };

    result
}

fn get_reservations<'a>() -> HashMap<&'a str, ItemMap> {
    let mut items = HashMap::new();
    items.insert("aaaa", 2);
    items.insert("bbbb", 2);

    let mut reservations = HashMap::new();
    reservations.insert("1234", items);

    reservations
}

mod resource;
use resource::Resource;

mod consumer;
use consumer::Consumer;

mod journal;
use journal::Journal;

mod database;
use database::Database;
use database::Command;

struct Input {
    stdin: StdinReader,
}

impl Input {
    fn new() -> Input {
        Input { stdin: old_io::stdin() }
    }
}

impl Iterator for Input {
    type Item = Result<Command, String>;

    fn next(&mut self) -> Option<Result<Command, String>> {
        println!("Enter request: (<consumer_id resource_id=quantity resource_id=quantity)");
        let input = self.stdin.read_line().ok().expect("Failed to read line");
        let parts =  &input.splitn(1, ' ').collect::<Vec<&str>>();

        let result: Result<Command, String> = match parts[0] {
            "STOCK" => Resource::new_from_string(parts[1]),
            "CONSUME" => Consumer::new_from_string(parts[1]),
            _ => Err("Invalid command.".to_string()),
        };

        Some(result)
    }
}

mod commands;

fn main() {
    let mut resources: ResourceMap = HashMap::new();
    let mut consumers: ConsumerMap = HashMap::new();

    let mut journal = Journal::new();
    for command in journal.iter() {
        command.process(&resources, &consumers);
        println!("{:?}", command);
    }

    for line in Input::new() {
        match line {
            Err(why)    => println!("{}", why),
            Ok(command) => {
                journal.write(&command);
                let result = command.process(&resources, &consumers);
                println!("{:?}", result);
            },
        }
    }

    // let reservations = get_reservations();

    // let mut input = HashMap::new();
    // input.insert("aaaa", 2);
    // input.insert("cccc", 1);

    // let existing = match reservations.get("1234") {
    //     Some(v) => diff(v, &input),
    //     None => input,
    // };

    // println!("{:?}", existing);
}
