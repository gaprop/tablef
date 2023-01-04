use csv::{ReaderBuilder, Reader};
use prettytable::Table;
use clap::Parser;
use std::io;
use std::io::Stdin;

extern crate prettytable;

/// A program to pretty print tables.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// Set the style spec for every cell.
    /// The syntax for the style spec looks like this : FrBybl which means Foreground red Background yellow bold left.
    /// List of supported specifiers :
    /// F : Foreground (must be followed by a color specifier)
    /// B : Background (must be followed by a color specifier)
    /// H : Horizontal span (must be followed by a number)
    /// b : bold
    /// i : italic
    /// u : underline
    /// c : Align center
    /// l : Align left
    /// r : Align right
    /// d : default style
    /// List of color specifiers :
    /// r : Red
    /// b : Blue
    /// g : Green
    /// y : Yellow
    /// c : Cyan
    /// m : Magenta
    /// w : White
    /// d : Black
    /// And capital letters are for bright colors. Eg :
    /// R : Bright Red
    /// B : Bright Blue
    /// … and so on …
    #[arg(short, long, value_name = "SPEC", verbatim_doc_comment)]
    all: Option<String>,

    /// Set the style for a specific column, where INDEX is the index of the clumn
    #[arg(short, long, value_name = "INDEX,SPEC", action = clap::ArgAction::Append)]
    column: Option<Vec<String>>,

    /// Set the style spec for a specific row, where INDEX is the index of the row
    #[arg(short, long, value_name = "INDEX,SPEC", action = clap::ArgAction::Append)]
    row: Option<Vec<String>>,

    /// Set the delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char
}


fn read_data(delimiter: u8) -> Reader<Stdin> {
    ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(io::stdin())
}

fn main() {
    let cli = Cli::parse();
    let delimiter: u8 = cli.delimiter
                       .to_string()
                       .as_bytes()[0];
    let mut data = read_data(delimiter);
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
            let mut column = column.split(cli.delimiter);
            let index = column
                     .next()
                     .expect("The first value has to be the number of the column that has to be changed")
                     .parse::<usize>()
                     .expect("The first value has to be the number of the column that has to be changed");
            let spec = column.next().expect("There was no second value in a -c");
            for cell in table.column_iter_mut(index) {
                *cell = cell.clone().style_spec(spec);
            }
        }
    }

    if let Some(rows) = cli.row.as_deref() {
        for row in rows {
            let mut row = row.split(cli.delimiter);
            let index = row
                     .next()
                     .expect("The first value has to be the number of the row that has to be changed")
                     .parse::<usize>()
                     .expect("The first value has to be the number of the row that has to be changed");
            let spec = row.next().expect("There was no second value in a -r");
            let cells = table.get_mut_row(index)
                             .expect("Index out of bound in a -r");
            for cell in cells.iter_mut() {
                *cell = cell.clone().style_spec(spec);
            }
        }
    }
    table.printstd();
}
