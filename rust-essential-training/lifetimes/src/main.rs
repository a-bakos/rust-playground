/**
 * Lifetime annotation
 *
 * Tells compiler how the lifetimes of the input
 * parameters RELATE to each other. The lifetime of
 * the return reference will be as long as the
 * lifetime of the two input parameters.
 * If X and Y have different lifetimes, the compiler
 * will use the smaller of the two.
 *
 * The borrow checker then uses that lifetime to
 * validate that wherever the returned reference
 * gets used later, the value it points to will
 * still be alive.
 */
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct lifetime annotation
struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    // We dont need to annotate lifetime in the function
    // signature because of Lifetime Rule #3.
    //fn send_transmission(&self, msg: &str) -> &str {
    //    println!("Transmitting message: {}", msg);
    //    self.name
    //}

    // If we want to return the message, we need to annotate the
    // lifetime
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("Result is {}", result);

    // Struct lifetime
    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("Sender is {}", sender);
}
