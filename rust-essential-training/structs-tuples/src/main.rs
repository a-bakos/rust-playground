#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // Associated function:
    // Like a constructor!
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,    // default
            propellant: 0.0, // default
        }
    }
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

    let vehicle_name = vehicle.get_name();
    println!("Vehicle name with get_name {}", vehicle_name);
    println!("Starting propellant: {}", vehicle.propellant);
    vehicle.add_fuel(848.53);
    println!("Updated propellant: {}", vehicle.propellant);

    let constructed_vehicle = Shuttle::new("Endeavour");
    println!("Constructed Vehicle is {:?}", constructed_vehicle);
}
