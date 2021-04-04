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

/**
 * Tuple Structs
 */
struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // X, Y, Z coordinates
fn get_y(p: Point) -> u8 {
    p.1
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

    /**
     * Tuple structs
     */
    let red = Color(255, 0, 0);
    println!("Red first value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("Y is {:?}", y);

    /**
     * Challenge
     */
    let mut rectangle = Rectangle::new(150.0, 250.0);
    println!("Rectangle is {:?}", rectangle);
    let rect_area = rectangle.get_area();
    println!("Rectangle area is {}", rect_area);
    rectangle.scale(0.5);
    println!("Rectangle is {:?}", rectangle);

    let rect_2 = Rectangle { w: 1.2, h: 3.4 };
    println!("Rectangle is {:?}", rect_2);

    let triangle = Triangle { a: 1, b: 5, c: 2 };
    println!("Triangle is {:?}", triangle);
}

#[derive(Debug)]
struct Rectangle {
    w: f64,
    h: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.w * self.h
    }

    // Associated function (cos no self as param)
    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle { w, h }
    }

    fn scale(&mut self, scale: f64) {
        self.w *= scale;
        self.h *= scale;
    }
}

// Generic data type:
#[derive(Debug)]
struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}
