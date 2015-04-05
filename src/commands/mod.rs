use database::Database;
use std::io::{BufRead, BufStream, Read, Write};
use std::io;

mod stock;
use self::stock::Stock;

mod consume;
use self::consume::Consume;

pub fn from_str(args: &str) -> Result<Box<Command>, &'static str> {
    match args.trim().split(' ').collect::<Vec<&str>>().as_ref() {
        [name, args..] => match name {
            "STOCK"   => Stock::from_str(args),
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

pub struct CommandStream<T: Read + Write> {
    inner: BufStream<T>
}

impl<T: Read + Write> CommandStream<T> {
    pub fn new(inner: BufStream<T>) -> Self {
        CommandStream { inner: inner }
    }

    pub fn write(&mut self, command: String) -> io::Result<usize> {
        self.inner.write(command.as_bytes())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    pub fn next(&mut self) -> Option<Result<Box<Command>, &'static str>> {
        let mut buffer = String::new();
        match self.inner.read_line(&mut buffer) {
            Ok(0) | Err(_) => None,
            Ok(length) => Some(from_str(&buffer)),
        }
    }
}
