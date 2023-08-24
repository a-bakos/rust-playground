#[derive(Debug)]
pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut st = Stepper {
        curr: 1,
        step: 5,
        max: 100,
    };

    loop {
        match st.next() {
            Some(v) => println!("looping : {:?}", v),
            None => break,
        }
    }
}
