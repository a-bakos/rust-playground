// open csv
// parse csv rows
// sort draw numbers

#[derive(Debug)]
pub struct Draw {
    id: String, // draw number
    date: String,
    balls: Vec<u8>,
    set: u8,
    machine: String,
}
impl Draw {
    pub fn new(id: String, date: String, set: u8, machine: String) -> Draw {
        Draw {
            id,
            date,
            balls: vec![],
            set,
            machine,
        }
    }
}

#[derive(Debug)]
pub struct DrawHistory {
    all: Vec<Vec<u8>>,
}

fn main() {
    let draw_test = Draw::new(
        "2651".to_string(),
        "19-May-21".to_string(),
        4,
        "Lancelot".to_string(),
    );

    println!("{:?}", draw_test);
}
