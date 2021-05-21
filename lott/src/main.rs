use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
//use thiserror::Error;

// open csv
// parse csv rows
// sort draw numbers

const FILENAME: &str = "lotto-draw-history.csv";

#[derive(Debug)]
pub struct DrawRecord {
    id: String, // draw number
    date: String,
    balls: Vec<u8>,
    bonus: u8,
    set: u8,
    machine: String,
}
impl DrawRecord {
    pub fn new(id: String, date: String, bonus: u8, set: u8, machine: String) -> DrawRecord {
        DrawRecord {
            id,
            date,
            balls: vec![],
            bonus,
            set,
            machine,
        }
    }
}

#[derive(Debug)]
pub struct DrawRecords {
    all: Vec<Vec<u8>>,
}
impl DrawRecords {
    pub fn new() -> DrawRecords {
        DrawRecords { all: vec![] }
    }

    pub fn add(&mut self, numbers: Vec<u8>) {
        self.all.push(numbers);
    }
}

// Handling file
// std::path::PathBuf
fn load_records(filename: PathBuf) -> std::io::Result<DrawRecords> {
    // return early with ? if there's a problem
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_records(buffer))
}

fn parse_records(buffer: String) -> DrawRecords {
    let mut recs = DrawRecords::new();
    for (_num, record) in buffer.split('\n').enumerate() {
        // .enumerate() gives us a tuple, with iteration number and the current data
        if record != "" {
            match parse_record(record) {
                Some(record) => recs.add(record.balls),
                None => println!("Err"),
            }
        }
    }
    recs
}

fn parse_record(record: &str) -> Option<DrawRecord> {
    let fields: Vec<&str> = record.split(',').collect();

    let date = match fields.get(0) {
        Some(date) => date.to_string(),
        None => "".to_string(),
    };
    let bonus = match fields.get(7) {
        Some(bonus) => bonus.trim().parse::<u8>().ok()?,
        None => 0,
    };
    let set = match fields.get(8) {
        Some(set) => set.trim().parse::<u8>().ok()?,
        None => 0,
    };
    let machine = match fields.get(9) {
        Some(machine) => machine.to_string(),
        None => "".to_string(),
    };
    let id = match fields.get(10) {
        Some(id) => id.to_string(),
        None => "".to_string(),
    };

    let ball_1 = fields[1].trim().parse::<u8>().ok()?;
    let ball_2 = fields[2].trim().parse::<u8>().ok()?;
    let ball_3 = fields[3].trim().parse::<u8>().ok()?;
    let ball_4 = fields[4].trim().parse::<u8>().ok()?;
    let ball_5 = fields[5].trim().parse::<u8>().ok()?;
    let ball_6 = fields[6].trim().parse::<u8>().ok()?;

    let mut balls = Vec::new();
    balls.push(ball_1);
    balls.push(ball_2);
    balls.push(ball_3);
    balls.push(ball_4);
    balls.push(ball_5);
    balls.push(ball_6);

    Some(DrawRecord {
        date,
        balls,
        bonus,
        set,
        machine,
        id,
    })
}

fn main() {
    let draw_test = DrawRecord::new(
        "2651".to_string(),
        "19-May-21".to_string(),
        10, //bonus
        4,
        "Lancelot".to_string(),
    );

    println!("{:?}", draw_test);
}
