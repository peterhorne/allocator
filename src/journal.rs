use std::old_path::Path;
use std::old_io::{Writer, Buffer, BufferedReader, File, Append, Write, IoResult};
use commands;
use commands::Command;

pub struct Journal {
    path: Path,
}

impl Journal {
    pub fn new() -> Journal {
        Journal { path: Path::new("./tmp/journal.txt") }
    }

    pub fn write<T: Command>(&mut self, command: &T) -> IoResult<()> {
        let mut file = try!(File::open_mode(&self.path, Append, Write));
        file.write_str(&command.to_string())
    }

    pub fn iter(&self) -> JournalIter {
        let file = BufferedReader::new(File::open(&self.path));
        JournalIter { file: file  }
    }
}

struct JournalIter {
    file: BufferedReader<IoResult<File>>,
}

impl Iterator for JournalIter {
    type Item = Result<Box<Command>, &'static str>;

    fn next(&mut self) -> Option<Result<Box<Command>, &'static str>> {
        match self.file.read_line() {
            Ok(line) => Some(commands::from_str(&line)),
            Err(why) => { println!("{}", why); None },
        }
    }
}
