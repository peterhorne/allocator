use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

mod journal;
use journal::Journal;

mod input;
use input::Input;

mod commands;
use commands::{Command, ResourceMap, ConsumerMap};

fn main() {
    let mut resources: ResourceMap = HashMap::new();
    let mut consumers: ConsumerMap = HashMap::new();
    let mut journal = Journal::new();

    for line in journal.iter() {
        line.ok()
            .expect("Journal is corrupt")
            .process(&mut resources, &mut consumers);
    }

    println!("Resources: {:?}", resources);
    println!("Consumers: {:?}", consumers);

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
