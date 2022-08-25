use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;

/// A simple program to generate a markdown &table for the user
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of columns to include
    #[clap(short, long, value_parser, default_value_t = 1)]
    cols: u8,

    /// Number of rows to include
    #[clap(short, long, value_parser, default_value_t = 1)]
    rows: u8,

    /// Saves the table to the clipboard as well as printing to stdout [default: false]
    #[clap(short = 'x', long, value_parser, default_value_t = false)]
    copy: bool,
}

const COL_SEPARATOR: char = '|';

fn main() {
    let args = Args::parse();

    // parse headers
    let header_row = (0..args.cols)
        .map(|_index| "Foo")
        .collect::<Vec<&str>>()
        .join(&COL_SEPARATOR.to_string())
        .to_string();

    // generate header row
    let header = format!("{}{}{}", COL_SEPARATOR, header_row, COL_SEPARATOR);

    let separator_row = (0..args.cols)
        .map(|_| "---")
        .collect::<Vec<&str>>()
        .join(&COL_SEPARATOR.to_string())
        .to_string();

    // generate table separator
    let separator_str = format!("{}{}{}", COL_SEPARATOR, separator_row, COL_SEPARATOR);

    // generate table body
    let mut table_rows: Vec<String> = Vec::new();

    for _ in 0..args.rows {
        let row = (0..args.cols)
            .map(|_| "Foo")
            .collect::<Vec<&str>>()
            .join(&COL_SEPARATOR.to_string())
            .to_string();

        table_rows.push(format!("{}{}{}", COL_SEPARATOR, row, COL_SEPARATOR));
    }

    let table_body = table_rows.join("\n");

    // generate whole table
    let table = format!("{}\n{}\n{}", header, separator_str, table_body);

    // output table
    if args.copy {
        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

        match clipboard.set_contents(table) {
            Ok(_) => println!(
                "{} {}",
                "Success!".green().bold(),
                "The table was copied to your clipboard",
            ),
            Err(_) => todo!(),
        }
    } else {
        println!("{}", table);
    }
}
