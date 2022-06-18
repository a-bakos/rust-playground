use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use thiserror::Error;

const FILENAME: &str = "draw-history.csv";
const MAX_NUM: u8 = 60;

// dedicated error type for parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("ID must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),

    #[error("Empty record")]
    EmptyRecord,

    #[error("Invalid number: {0}")]
    InvalidNumber(u8),
}

#[derive(Debug)]
pub struct DrawRecord {
    pub id: String,
    pub date: String,
    pub balls: Vec<u8>,
    pub bonus: u8,
    pub set: u8,
    pub machine: String,
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
    pub all: Vec<Vec<u8>>,
    pub all_1: Vec<u8>,
    pub all_2: Vec<u8>,
    pub all_3: Vec<u8>,
    pub all_4: Vec<u8>,
    pub all_5: Vec<u8>,
    pub min_set: Vec<u8>,
    pub max_set: Vec<u8>,
    pub most_common_set: Vec<u8>,
}
impl DrawRecords {
    pub fn new() -> DrawRecords {
        DrawRecords {
            all: vec![],
            all_1: vec![],
            all_2: vec![],
            all_3: vec![],
            all_4: vec![],
            all_5: vec![],
            min_set: vec![],
            max_set: vec![],
            most_common_set: vec![],
        }
    }

    pub fn add(&mut self, numbers: Vec<u8>) {
        self.all.push(numbers);
    }

    pub fn extract_nums(&mut self) {
        for record in &self.all {
            for (index, _num) in record.iter().enumerate() {
                let the_num = |x: usize| record[x as usize];
                match index {
                    0 => self.all_1.push(the_num(index)),
                    1 => self.all_2.push(the_num(index)),
                    2 => self.all_3.push(the_num(index)),
                    3 => self.all_4.push(the_num(index)),
                    4 => self.all_5.push(the_num(index)),
                    _ => println!("Nope"),
                }
            }
        }

        self.all_1.sort();
        self.all_2.sort();
        self.all_3.sort();
        self.all_4.sort();
        self.all_5.sort();

        // min
        // max
        // most common
        // count each

        let print_sets = |title: &str, set: &Vec<u8>| {
            print!("{}", title);
            for num in set {
                print!("{:-2} ", num);
            }
            println!("");
        };

        print_sets("All 1st: ", &self.all_1);
        print_sets("All 2nd: ", &self.all_2);
        print_sets("All 3rd: ", &self.all_3);
        print_sets("All 4th: ", &self.all_4);
        print_sets("All 5th: ", &self.all_5);
    }

    pub fn get_max_from_sets(&mut self) {
        let get_max_from = |x: &Vec<u8>| match x.iter().max() {
            Some(max) => *max,
            None => 0,
        };

        self.max_set.push(get_max_from(&self.all_1));
        self.max_set.push(get_max_from(&self.all_2));
        self.max_set.push(get_max_from(&self.all_3));
        self.max_set.push(get_max_from(&self.all_4));
        self.max_set.push(get_max_from(&self.all_5));

        println!("The max set: {:?}", self.max_set);
    }

    pub fn get_min_from_sets(&mut self) {
        let get_min_from = |x: &Vec<u8>| match x.iter().min() {
            Some(min) => *min,
            None => 0,
        };

        self.min_set.push(get_min_from(&self.all_1));
        self.min_set.push(get_min_from(&self.all_2));
        self.min_set.push(get_min_from(&self.all_3));
        self.min_set.push(get_min_from(&self.all_4));
        self.min_set.push(get_min_from(&self.all_5));

        println!("The min set: {:?}", self.min_set);
    }

    pub fn occurences_in_sets(&mut self) {
        let count_occurences = |x: &Vec<u8>| {
            let mut count = HashMap::new();
            for num in x {
                let num_count = match count.get(num) {
                    Some(num_val) => num_val + 1,
                    None => 1,
                };
                count.insert(num, num_count);
            }

            let mut most_common_v: u8 = 0;
            let mut most_common_ks: Vec<u8> = vec![];
            for (_k, &v) in count.iter() {
                if v > most_common_v {
                    most_common_v = v;
                }
            }
            for (&k, &v) in count.iter() {
                if v == most_common_v {
                    if most_common_ks.contains(&k) {
                        continue;
                    } else {
                        most_common_ks.push(*k);
                    }
                }
            }

            println!("");
            println!("MOST COMMON NUMBERS {:?}", most_common_ks);
        };

        count_occurences(&self.all_1);
        count_occurences(&self.all_2);
        count_occurences(&self.all_3);
        count_occurences(&self.all_4);
        count_occurences(&self.all_5);
    }
}

// Handling file
fn load_records(filename: &str) -> std::io::Result<DrawRecords> {
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
            Err(ParseError::InvalidNumber(num))
        }
    };

    let mut balls = Vec::new();
    balls.push(ball_number(&fields[1].to_string())?);
    balls.push(ball_number(&fields[2].to_string())?);
    balls.push(ball_number(&fields[3].to_string())?);
    balls.push(ball_number(&fields[4].to_string())?);
    balls.push(ball_number(&fields[5].to_string())?);

    balls.sort();

    Ok(DrawRecord {
        date,
        balls,
        bonus,
        set,
        machine,
        id,
    })
}

fn get_input() -> Option<String> {
    let mut buffer: String = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_string();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn menu() {
    let mut user_input: Option<String>;
    loop {
        let user_input = get_input();

        match user_input {
            Some(the_input) => {
                println!("{:?}", the_input);
            }
            None => continue,
        }
    }
}

fn run() -> Result<(), std::io::Error> {
    let mut recs = load_records(FILENAME)?;

    recs.extract_nums();
    recs.get_min_from_sets();
    recs.get_max_from_sets();

    recs.occurences_in_sets();

    Ok(for record in recs.all {
        for num in record {
            print!("{:-2} ", num);
        }
        println!("");
    })
}

fn main() {
    run();
    //menu();
}
