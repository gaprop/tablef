use csv::{ReaderBuilder, Reader, StringRecord};
use prettytable::Table;
use clap::Parser;
use std::error::Error;
use std::io;
use std::io::Stdin;
use std::process;
#[macro_use]
extern crate prettytable;

/// A program to pretty print tables
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the style spec for every cell
    #[arg(short, long)]
    all: bool,

    /// The spec style
    #[arg(short, long, value_name = "SPEC", action = clap::ArgAction::Append)]
    spec: Vec<String>,

    // /// Set the style spec for a row
    // #[arg(short, long, value_name = "SPEC")]
    // row: Option<String>,
}


fn read_data() -> Reader<Stdin> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin())
}

fn main() {
    let mut cli = Cli::parse();
    let mut data = read_data();
    let mut table = Table::from_csv(&mut data);

    if cli.all {
    // if let Some(all) = cli.all.as_deref() {
        let spec = cli.spec.pop().unwrap();
        for row in table.row_iter_mut() {
            for cell in row.iter_mut() {
                *cell = cell.clone().style_spec(&spec);
            }
        }
    }
    table.printstd();
}
