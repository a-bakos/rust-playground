use std::fs::File;
use std::io::Read;
use thiserror::Error;

const FILENAME: &str = "lotto-draw-history.csv";
const MAX_NUM: u8 = 60;

// dedicated error type for parsing
#[derive(Debug, Error)]
enum ParseError {
    #[error("ID must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),

    #[error("Empty record")]
    EmptyRecord,

    #[error("Wrong number: {0}")]
    WrongNumber(u8),
}

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
fn load_records(filename: &str) -> std::io::Result<DrawRecords> {
    // return early with ? if there's a problem
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_records(buffer))
}

fn parse_records(buffer: String) -> DrawRecords {
    let mut recs = DrawRecords::new();
    for (num, record) in buffer.split('\n').enumerate() {
        // Skip the header line
        if num == 0 {
            continue;
        }
        // .enumerate() gives us a tuple, with iteration number and the current data
        if record != "" {
            match parse_record(record) {
                Ok(record) => recs.add(record.balls),
                Err(e) => println!("Error on line {}: {}\n > \"{}\"\n", num + 1, e, record),
            }
        }
    }
    recs
}

fn parse_record(record: &str) -> Result<DrawRecord, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();

    let date = match fields.get(0) {
        Some(date) => date.to_string(),
        None => return Err(ParseError::EmptyRecord),
    };
    let bonus = match fields.get(7) {
        Some(bonus) => u8::from_str_radix(bonus, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    let set = match fields.get(8) {
        Some(set) => u8::from_str_radix(set, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    let machine = match fields.get(9) {
        Some(machine) => machine.to_string(),
        None => return Err(ParseError::EmptyRecord),
    };
    let id = match fields.get(10) {
        Some(id) => id.to_string(),
        None => return Err(ParseError::EmptyRecord),
    };

    let ball_number = |x: &String| -> Result<u8, ParseError> {
        let num = u8::from_str_radix(x, 10)?;
        if num > 0 && num < MAX_NUM {
            Ok(num)
        } else {
            Err(ParseError::WrongNumber(num))
        }
    };

    let mut balls = Vec::new();
    balls.push(ball_number(&fields[1].to_string())?);
    balls.push(ball_number(&fields[2].to_string())?);
    balls.push(ball_number(&fields[3].to_string())?);
    balls.push(ball_number(&fields[4].to_string())?);
    balls.push(ball_number(&fields[5].to_string())?);
    balls.push(ball_number(&fields[6].to_string())?);

    Ok(DrawRecord {
        date,
        balls,
        bonus,
        set,
        machine,
        id,
    })
}

fn main() -> Result<(), std::io::Error> {
    let recs = load_records(FILENAME)?;

    Ok(for record in recs.all {
        println!("{:?}", record);
    })
}
