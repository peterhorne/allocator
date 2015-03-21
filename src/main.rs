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

// mod resource;
// use resource::Resource;

// mod consumer;
// use consumer::Consumer;

mod journal;
use journal::Journal;

// mod database;
// use database::Database;
// use database::Command;

struct Input {
    stdin: StdinReader,
}

impl Input {
    fn new() -> Input {
        Input { stdin: old_io::stdin() }
    }
}

impl Iterator for Input {
    type Item = Result<Box<Command>, &'static str>;

    fn next(&mut self) -> Option<Result<Box<Command>, &'static str>> {
        println!("Enter request: (<consumer_id resource_id=quantity resource_id=quantity)");
        match self.stdin.read_line() {
            Ok(line) => Some(commands::deserialise(&line)),
            Err(_)   => None,
        }
    }
}

mod commands;
use commands::{Command, ResourceMap, ConsumerMap};

fn main() {
    let mut resources: ResourceMap = HashMap::new();
    let mut consumers: ConsumerMap = HashMap::new();

    let mut journal = Journal::new();
    for line in journal.iter() {
        match line {
            Ok(command) => {
                let result = command.process(&mut resources, &mut consumers);
                println!("{:?}", result.serialise());
            },
            Err(why) => println!("{}", why),
        };
    }

    // for line in Input::new() {
    //     match line {
    //         Err(why)    => println!("{}", why),
    //         Ok(command) => {
    //             journal.write(&command);
    //             let result = command.process(&resources, &consumers);
    //             println!("{:?}", result);
    //         },
    //     }
    // }

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
