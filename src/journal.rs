use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Lines, Write};

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

    pub fn write<T: Command>(&mut self, command: &T) -> io::Result<()> {
        let mut options = OpenOptions::new();
        options.append(true);
        let mut file = try!(options.open(&self.path));
        file.write_all(command.to_string().as_bytes())
    }

    pub fn iter(&self) -> CommandReader<BufReader<File>> {
        let file = File::open(&self.path).ok().expect("Couldn't open journal.");
        let journal = BufReader::new(file);
        CommandReader::new(journal)
    }
}
