#[derive(Debug, Clone)]
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

    let vehicle_2 = Shuttle {
        name: String::from("Discovery"),
        // this tells rust that any fields that are not explicitly
        // set should be the same as the previous "vehicle"
        // -- These two structs are separate instances. If we make changes
        // to the first vehicle, after the second one has been instantiated
        // using those values, those changes will not be reflected in the second
        // vehicle.
        ..vehicle
    };

    let vehicle_3 = Shuttle { ..vehicle.clone() };

    vehicle.name = String::from("Atlantis");
    vehicle.crew_size = 6;
    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle 2 is {:?}", vehicle_2);
    println!("Vehicle 3 is {:?}", vehicle_3);
}
