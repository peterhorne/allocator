use std::old_io::{BufferedReader, File, Append, Write, IoResult};
use consumer::Consumer;

pub struct Journal {
    path: Path,
}

impl Journal {
    pub fn new() -> Journal {
        Journal { path: Path::new("/tmp/allocator-journal.txt") }
    }

    pub fn write(&mut self, reservation: Consumer) -> bool {
        let mut file = match File::open_mode(&self.path, Append, Write) {
            Err(why) => {
                println!("{}", why);
                return false;
            },
            Ok(file) => file,
        };

        let allocations = reservation.allocations.iter()
            .map(|(stock_id, quantity)| format!("{}={}", stock_id, quantity))
            .collect::<Vec<String>>()
            .connect(" ");

        let line = format!("{}\n", [reservation.id, allocations].connect(" "));

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
    type Item = Consumer;

    fn next(&mut self) -> Option<Consumer> {
        match self.file.read_line() {
            Ok(line) => Consumer::new_from_string(line),
            Err(_) => None,
        }
    }
}
