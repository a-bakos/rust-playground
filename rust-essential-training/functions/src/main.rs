fn main() {
    let result = square(13);
    println!("Result is {:?}", result);

    let celsius: f32 = 28.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!(
        "The temperature is {} in Celsius and {:.2} in Fahrenheit",
        celsius, fahrenheit
    );
}

fn square(x: i32) -> (i32, i32) {
    println!("Squaring {}", x);
    return (x, x * x);
}

fn celsius_to_fahrenheit(c_temp: f32) -> f32 {
    (1.8 * c_temp) + 32 as f32
}
