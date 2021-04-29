use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

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

    // not a borrow, so once we use this function, the "inner" data will be gone
    // and we won't be able to access Records struct anymore
    fn into_vec(mut self) -> Vec<Record> {
        // .drain() will go through the hashmap and will drain all the values into
        // something else. so it's moving everything out of the "inner" hashmap
        // into another structure
        // kv is a tuple
        let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
        records.sort_by_key(|rec| rec.id);
        records
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        match ids.pop() {
            // pop() takes the last item off
            Some(id) => id + 1,
            None => 1,
        }
    }

    fn search(&self, name: &str) -> Vec<&Record> {
        // borrow Record because we're borrowing self
        self.inner
            .values()
            .filter(|rec| rec.name.to_lowercase().contains(&name.to_lowercase())) // for .contains() won't have to be an exact match
            .collect()
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }

    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.inner.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email,
            },
        );
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
    for (num, record) in buffer.split('\n').enumerate() {
        // .enumerate() gives us a tuple, with iteration number and the current data
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

fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();
    let id = match fields.get(0) {
        // trying to turn the string representation of the id to a number we can work with
        // id is the string, 10 is base-10 numbers
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    // double asterisk in filter:
    // - first, the fields vector has our reference to a string
    // - and get() gives us another reference (so two && ampersands so far)
    // - finally, when we call filter, the closure parameter "name" will be
    // referenced again
    // - so we have to dereference it twice with **
    let name = match fields.get(1).filter(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };
    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");

    Ok(Record { id, name, email })
}

// crate: structopt
// allows you to create a structure, and it'll automatically
// turn it into command line options that you can use
#[derive(StructOpt, Debug)]
#[structopt(about = "project 2: contact manager")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}
#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    List {},
    Search {
        query: String,
    },
    Remove {
        id: i64,
    },
    Edit {
        id: i64,
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("an error occured: {}", e);
    }
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
        }
        // .. is to ignore all information in the List
        Command::List { .. } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record);
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            let results = recs.search(&query);
            if results.is_empty() {
                println!("No records found.");
            } else {
                for rec in results {
                    println!("{:?}", rec);
                }
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("Record deleted");
            } else {
                println!("Record not found");
            }
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
        }
    }
    Ok(())
}

fn save_records(filename: PathBuf, records: Records) -> std::io::Result<()> {
    let mut file = OpenOptions::new() // openoptions to configure how to open the file
        .write(true)
        .truncate(true) // erase the file
        .open(filename)?;

    // write() accepts raw bytes
    file.write(b"id,name,email\n")?; // "b" tells the compiler to use bytes instead of a string

    // into_iter() creates an iterator and moves the values out of the vector
    // into the loop
    for record in records.into_vec().into_iter() {
        let email = match record.email {
            Some(email) => email,
            None => "".to_string(),
        };
        let line = format!("{},{},{}\n", record.id, record.name, email); // {} release mode print
        file.write(line.as_bytes())?;
    }
    file.flush(); // flush makes a request to the system and it won't return until
                  // system's successfully written the data to the disk or it failed
                  // without it, the file won't be properly written
    Ok(())
}
