struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    fn freezing() -> Self {
        // better to use Self because if we change the
        // Struct's name, we don't have to change the return type
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

    // Challenge
    let small_dimension = Dimensions {
        mm_width: 250,
        mm_height: 165,
        mm_depth: 300,
    };
    let small_box = ShippingBox::new(3450, BoxColor::Black, small_dimension);
    small_box.info();
}

// Challenge

enum BoxColor {
    Yellow,
    Black,
    Brown,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Yellow => println!("Yellow"),
            BoxColor::Brown => println!("Brown"),
            BoxColor::Black => println!("Black"),
        }
    }
}

struct Dimensions {
    mm_width: i32,
    mm_height: i32,
    mm_depth: i32,
}
impl Dimensions {
    fn print(&self) {
        println!("Width: {}", self.mm_width);
        println!("Height: {}", self.mm_height);
        println!("Depth: {}", self.mm_depth);
    }
}

struct ShippingBox {
    color: BoxColor,
    g_weight: i32,
    dimension: Dimensions,
}

impl ShippingBox {
    fn new(g_weight: i32, color: BoxColor, dimension: Dimensions) -> Self {
        Self {
            g_weight,
            color,
            dimension,
        }
    }

    fn info(&self) {
        self.color.print();
        self.dimension.print();

        println!("Weight is: {}", self.g_weight);
    }
}
