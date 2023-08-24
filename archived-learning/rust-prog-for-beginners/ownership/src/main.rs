struct Item {
    id: i32,
    quantity: i32,
}

fn print_quantity(item: &Item) {
    println!("Quantity is: {}", item.quantity);
}

fn print_item_id(item: &Item) {
    println!("ID is: {}", item.id);
}

fn main() {
    let my_item = Item {
        id: 1,
        quantity: 10,
    };

    print_quantity(&my_item);
    print_item_id(&my_item);
}
