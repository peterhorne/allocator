use std::env;
// use std::result::Result;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

// ["reservation_id", "item_id=3", "item_id=2"]
fn parse_args(args: &Vec<String>) -> Result<Reservation, &str> {
    let reservation_id = args[1].clone(); // todo
    let mut reservation = Reservation { id: reservation_id, allocations: HashMap::new() };

    for arg in args.slice_from(2) {
        let split = arg.split_str("=").collect::<Vec<&str>>();
        let stock_id = split[0];
        let quantity: i32 = match split[1].parse() {
            Ok(num) => num,
            Err(_) => { return Err("It borked.") },
        };
        // (stock_id, quantity) = arg.split_str("=");
        // reservation.allocations.insert(stock_id, quantity);
    }


    Ok(reservation)
}

fn log_request(reservation: Reservation) {

}

// parse input -> Reservation
// write input to log
// parse log -> Database(Stocks, Reservations)
// return reservation details -> Reservation
fn main() {
    let args: Vec<_> = env::args().collect();
    let request = match parse_args(&args) {
        Ok(reservation) => reservation,
        Err(_) => {
            println!("Invalid arguments.");
            return;
        }
    };

    println!("{:?}", request.id);

    log_request(request);

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
