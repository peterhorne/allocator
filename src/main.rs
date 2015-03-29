#![feature(convert)]
#![feature(old_path)]
#![feature(old_io)]

mod database;
mod journal;
mod input;
mod commands;

use database::Database;
use journal::Journal;
use input::Input;
use commands::Command;

fn main() {
    let mut database = Database::new();
    let mut journal = Journal::new();

    for line in journal.iter() {
        line.ok()
            .expect("Journal is corrupt")
            .execute(&mut database);
    }

    println!("Database: {:?}", database);

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
