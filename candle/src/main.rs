#[derive(Debug)]
struct Candle {
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}

impl Candle {
    fn new() -> Candle {
        Candle {
            open: rand::random(),  // sets the init price
            high: rand::random(),  // Range: must be eq to or higher than open
            low: rand::random(),   // Range: eq to or lower than open
            close: rand::random(), // must be eq to any, and be within range
        }
    }
}

fn main() {
    let candle_1 = Candle {
        open: 0.85510,
        close: 0.85807,
        high: 0.85870,
        low: 0.85381,
    };
    let candle_2 = Candle::new();

    println!("The first candle is {:?}", candle_1);
    println!("The second candle is {:?}", candle_2);
}
