use std::old_io::{BufferedReader, File, Append, Write, IoResult};
use commands::Command;
use commands;

pub struct Journal {
    path: Path,
}

impl Journal {
    pub fn new() -> Journal {
        Journal { path: Path::new("./tmp/journal.txt") }
    }

    pub fn write<T: Command>(&mut self, command: &T) -> bool {
        let mut file = match File::open_mode(&self.path, Append, Write) {
            Err(why) => {
                println!("{}", why);
                return false;
            },
            Ok(file) => file,
        };

        // let resources = consumer.resources.iter()
        //     .map(|(stock_id, quantity)| format!("{}={}", stock_id, quantity))
        //     .collect::<Vec<String>>()
        //     .connect(" ");

        // let line = format!("{} {}\n", consumer.id, resources);

        let line = command.serialise();

        match file.write_str(&line) {
            Err(why) => {
                println!("{}", why);
                false
            },
            Ok(_) => true
        }
    }

    pub fn iter(&self) -> JournalIter {
        let mut file = BufferedReader::new(File::open(&self.path));
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
