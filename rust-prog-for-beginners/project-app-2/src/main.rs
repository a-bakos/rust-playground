// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

use std::collections::HashMap;
use std::io::File;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>, // K V
}

impl Records {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }
}

// dedicated error type for parsing
#[derive(Debug, Error)]
enum ParseError {
    #[error("ID must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),

    #[error("Empty record")]
    EmptyRecord,

    #[error("Missing fields: {0}")]
    MissingField(String),
}

// Handling file
// std::path::PathBuf
fn load_records(filename: PathBuf, verbose: bool) -> std::io::Result<Records> {
    // return early with ? if there's a problem
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_records(buffer, verbose))
}

fn parse_records(buffer: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (num, record) in recs.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(record) => recs.add(record),
                Err(e) => {
                    if verbose {
                        println!("Error on line {}: {}\n > \"{}\"\n", num + 1, e, record);
                    }
                }
            }
        }
    }
    recs
}

fn parse_record(record) {

}

fn main() {
    println!("Hello, world!");
}
