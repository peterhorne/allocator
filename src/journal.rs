use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines, Write};
use std::io;

use commands;
use commands::Command;

struct CommandReader<T: BufRead> {
    lines: Lines<T>,
}

impl<T: BufRead> CommandReader<T> {
    fn new(reader: T) -> Self {
        CommandReader { lines: reader.lines() }
    }
}

impl<T: BufRead> Iterator for CommandReader<T> {
    type Item = Result<Box<Command>, &'static str>;

    fn next(&mut self) -> Option<Result<Box<Command>, &'static str>> {
        match self.lines.next() {
            Some(line) => match line {
                Ok(line) => Some(commands::from_str(&line)),
                Err(_)   => None,
            },
            None => None,
        }
    }
}

pub struct Journal {
    path: String,
}

impl Journal {
    pub fn new(path: String) -> Journal {
        Journal { path: path }
    }

    pub fn write(&mut self, command: &String) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .ok()
            .expect("Couldn't open journal.");

        try!(file.write_all(command.as_bytes()));
        file.flush()
    }

    pub fn iter(&self) -> CommandReader<BufReader<File>> {
        let file = File::open(&self.path).ok().expect("Couldn't open journal.");
        let journal = BufReader::new(file);
        CommandReader::new(journal)
    }
}
