#![feature(convert)]
#![feature(slice_patterns)]

use std::io::{BufStream, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod database;
mod journal;
mod input;
mod commands;

use database::Database;
use journal::Journal;
// use input::Input;
use commands::{Command, CommandStream};

fn main() {
    let mut database = Database::new();
    let journal = Journal::new("./tmp/journal.txt".to_string());

    for line in journal.iter() {
        line.ok()
            .expect("Journal is corrupt")
            .execute(&mut database);
    }

    let journal_mutex = Arc::new(Mutex::new(journal));
    let db_mutex = Arc::new(Mutex::new(database));

    let socket = "127.0.0.1:8000";
    let listener = TcpListener::bind(socket).unwrap();
    println!("Listing on {}", socket);

    for connection in listener.incoming() {
        match connection {
            Ok(mut connection) => {
                let connection = CommandStream::new(BufStream::new(connection));
                let journal = journal_mutex.clone();
                let database = db_mutex.clone();
                thread::spawn(move|| {
                    handle_connection(connection, database, journal);
                });
            }
            Err(e) => { println!("Error: {:?}", e); }
        }
    }
}

fn handle_connection<T: Read + Write>(mut connection: CommandStream<T>, db_mutex: Arc<Mutex<Database>>, journal_mutex: Arc<Mutex<Journal>>) {
    loop {
        match connection.next() {
            Some(Ok(command)) => {
                let mut database = db_mutex.lock().unwrap();
                let mut journal = journal_mutex.lock().unwrap();
                let result = command.execute(&mut *database);
                // journal.write(&result);
                connection.write(result.to_string());
                connection.write('\n'.to_string());
                connection.flush();
            }
            Some(Err(why)) => {
                connection.write(why.to_string());
                connection.write('\n'.to_string());
                connection.flush();
            }
            _ => { break; }
        }
    }
}
