use csv::{ReaderBuilder, Reader, StringRecord};
use std::error::Error;
use std::io;
use std::io::Stdin;
use std::process;
#[macro_use]
extern crate prettytable;

use prettytable::Table;

fn read_data() -> Reader<Stdin> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin())
}

fn main() {
    let mut data = read_data();
    let table = Table::from_csv(&mut data);
    table.printstd();
}
