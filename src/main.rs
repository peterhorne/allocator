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

mod consumer;
use consumer::{Consumer, AllocationMap, Quantity};


mod journal;
use journal::Journal;

mod database;
use database::Database;

struct ConsumerRequests {
    stdin: StdinReader,
}

impl ConsumerRequests {
    fn new() -> ConsumerRequests {
        ConsumerRequests { stdin: old_io::stdin() }
    }
}

impl Iterator for ConsumerRequests {
    type Item = Consumer;

    fn next(&mut self) -> Option<Consumer> {
        println!("Enter request: (<consumer_id resource_id=quantity resource_id=quantity)");
        let input = self.stdin.read_line().ok().expect("Failed to read line");
        Consumer::new_from_string(input)
    }
}

fn main() {
    let mut database = Database::new();
    let mut journal = Journal::new();
    for reservation in journal.iter() {
        println!("{:?} {:?}", reservation.id, reservation.allocations);
    }

    let requests = ConsumerRequests::new();

    for request in requests {
        journal.write(request);
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
