enum DrinkFlavours {
    Strawberry,
    Pineapple,
    Coconut,
}

struct Drink {
    flavour: DrinkFlavours,
    volume: f64,
}

fn main() {
    let my_drink = Drink {
        flavour: DrinkFlavours::Coconut,
        volume: 500.0,
    };

    let my_drink_flavour = match my_drink.flavour {
        DrinkFlavours::Strawberry => "Strawberry",
        DrinkFlavours::Pineapple => "Pineapple",
        DrinkFlavours::Coconut => "Coconut",
    };

    println!(
        "My drink tastes {}-y and is {} ml.",
        my_drink_flavour, my_drink.volume
    );
}
