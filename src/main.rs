extern crate csv;
extern crate custom_error;

mod transaction;
mod order_book_map;

use std::fs::File;
use std::path::Path;
use transaction::*;
use order_book_map::OrderBookMap;

fn main() {
    let (path, trade) = match parse_args(std::env::args().collect::<Vec<String>>()) {
        Ok(s) => s,
        Err(usage) => {
            eprintln!("{}", usage);
            return
        }
    };

    println!("{:?}", path);
    let in_file = File::open(&path).expect(&format!(
        "File '{}' does not exist",
        path
    ));

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(in_file);

    let mut order_book = OrderBookMap::new(trade);

    for result in reader.records() {
        let transaction = Transaction::from_string_record(&result.unwrap());
        order_book.do_transaction(transaction.unwrap());
    }
}

fn parse_args(args: Vec<String>) -> Result<(String, bool), String> {
    match args.len() {
        2 => {
            let path = Path::new(&args[1]);
            if path.exists() {
                Ok((args[1].to_string(), false))
            } else {
                Err(format!("input file '{}' does not exist", args[1]))
            }
        },
        3 => {
            let path = Path::new(&args[1]);
            if path.exists() {
                match &*args[2] {
                    "-t" => Ok((args[1].to_string(), true)),
                    _ => Err(format!("usage: {} <path/to/input.csv> [-t]", args[0],)),
                }
            } else {
                Err(format!("input file '{}' does not exist", args[1]))
            }
        }
        _ => Err(format!("usage: {} <path/to/input.csv> [-t]", args[0],))
    }
}
