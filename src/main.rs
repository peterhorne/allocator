use std::env;
// use std::result::Result;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_io::{File, Append, Write};

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

type Quantity = i32;
type AllocationMap = HashMap<String, Quantity>;

struct Reservation {
    id: String,
    allocations: AllocationMap,
}

/// Parses cli args of the format: ./allocator 1234 aaaa=2 bbbb=2
/// 
/// Where:
///     1234 = Reservation ID
///     aaaa = Stock Item ID
fn parse_request<'a>(args: Vec<String>) -> Result<Reservation, &'a str> {
    let reservation_id = match args.get(0) {
        Some(id) => id.clone(),
        None => { return Err("Missing reservation id.") },
    };

    let mut allocations = HashMap::new();
    for arg in &args[1..] {
        let mut split = arg.split("=");

        let stock_id = match split.next() {
            Some(id) => id,
            None => { return Err("Missing stock id.") },
        };

        let quantity: Quantity = match split.next() {
            Some(num) => {
                match num.parse() {
                    Ok(num) => num,
                    Err(_) => { return Err("Quantity is not an integer.") },
                }
            },
            None => { return Err("Missing or quantity.") },
        };

        allocations.insert(stock_id.to_string(), quantity);
    }

    Ok(Reservation { id: reservation_id, allocations: allocations })
}

fn write_to_journal(reservation: Reservation) -> bool {
    let path = Path::new("/tmp/allocator-journal.txt");

    let mut file = match File::open_mode(&path, Append, Write) {
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

// parse input -> Reservation
// write input to log
// parse log -> Database(Stocks, Reservations)
// return reservation details -> Reservation
fn main() {
    let args = env::args().skip(1).collect();
    let request = match parse_request(args) {
        Ok(reservation) => reservation,
        Err(_) => {
            println!("Invalid arguments.");
            return;
        }
    };

    println!("{:?}", request.id);
    println!("{:?}", request.allocations);

    if !write_to_journal(request) {
        println!("Failed to write to journal.");
        return;
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
