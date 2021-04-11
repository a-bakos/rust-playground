struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_c: -10.0 }
    }

    fn print_temp(&self) {
        println!("The temperature is {} C", self.degrees_c);
    }
}

fn main() {
    let temp = Temperature { degrees_c: 25.0 };
    temp.print_temp();

    let cold = Temperature::freezing();
    cold.print_temp();
}
