use csv::{ReaderBuilder, Reader};
use prettytable::Table;
use clap::Parser;
use std::io;
use std::io::Stdin;

extern crate prettytable;

/// A program to pretty print tables
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the style spec for every cell
    #[arg(short, long, value_name = "SPEC")]
    all: Option<String>,

    /// Set the style for a specific column
    #[arg(short, long, value_name = "SPEC", action = clap::ArgAction::Append)]
    column: Option<Vec<String>>,

    /// Set the style spec for a specific row
    #[arg(short, long, value_name = "SPEC", action = clap::ArgAction::Append)]
    row: Option<Vec<String>>,
}


fn read_data() -> Reader<Stdin> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin())
}

fn main() {
    let cli = Cli::parse();
    let mut data = read_data();
    let mut table = Table::from_csv(&mut data);

    if let Some(all) = cli.all.as_deref() {
        for row in table.row_iter_mut() {
            for cell in row.iter_mut() {
                *cell = cell.clone().style_spec(all);
            }
        }
    }

    if let Some(columns) = cli.column.as_deref() {
        for column in columns {
            let mut column = column.split(",");
            let id = column
                     .next()
                     .expect("The first value has to be the number of the column that has to be changed")
                     .parse::<usize>()
                     .expect("The first value has to be the number of the column that has to be changed");
            let spec = column.next().expect("There was no second value in a -c");
            for cell in table.column_iter_mut(id) {
                *cell = cell.clone().style_spec(spec);
            }
        }
    }

    if let Some(rows) = cli.row.as_deref() {
        for row in rows {
            let mut row = row.split(",");
            let id = row
                     .next()
                     .expect("The first value has to be the number of the row that has to be changed")
                     .parse::<usize>()
                     .expect("The first value has to be the number of the row that has to be changed");
            let spec = row.next().expect("There was no second value in a -r");
            let cells = table.get_mut_row(id)
                             .expect("Index out of bound in a -r");
            for cell in cells.iter_mut() {
                *cell = cell.clone().style_spec(spec);
            }
        }
    }
    table.printstd();
}
