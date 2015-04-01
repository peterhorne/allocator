// use std::old_io;
// use std::old_io::stdio::{StdinReader};
// use commands;
// use commands::Command;

// pub struct Input {
//     stdin: StdinReader,
// }

// impl Input {
//     fn new() -> Input {
//         Input { stdin: old_io::stdin() }
//     }
// }

// impl Iterator for Input {
//     type Item = Result<Box<Command>, &'static str>;

//     fn next(&mut self) -> Option<Result<Box<Command>, &'static str>> {
//         println!("Enter request: (<consumer_id resource_id=quantity resource_id=quantity)");
//         match self.stdin.read_line() {
//             Ok(line) => Some(commands::from_str(&line)),
//             Err(_)   => None,
//         }
//     }
// }
