#[derive(Debug)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Alien"),
        crew_size: 8,
        propellant: 83265.5,
    };
    println!("Name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("Vehicle is {:?}", vehicle);
}
