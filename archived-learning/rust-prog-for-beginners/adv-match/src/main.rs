enum Ticket {
    Backstage(f64, String, bool),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let sold_tickets = vec![
        Ticket::Backstage(250.0, String::from("Jon"), true),
        Ticket::Vip(200.0, String::from("Mary")),
        Ticket::Standard(100.0),
    ];

    for ticket in sold_tickets {
        match ticket {
            Ticket::Backstage(price, name, _) => {
                // Ignore last parameter in the enum variant
                println!("Backstage Ticket is for: {}, cost: {}", name, price)
            }
            Ticket::Vip(price, name) => println!("VIP Ticket is for: {}, cost: {}", name, price),
            Ticket::Standard(price) => println!("Standard ticket, cost: {}", price),
        }
    }
}
