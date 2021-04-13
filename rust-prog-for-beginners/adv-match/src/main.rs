enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let sold_tickets = vec![
        Tickets::Backstage(250.0, String::from("Jon")),
        Tickets::Vip(200.0, String::from("Mary")),
        Tickets::Standard(100.0),
    ];

    for ticket in sold_tickets {
        match ticket {
            Tickets::Backstage(price, name) => {
                println!("Backstage Ticket is for: {}, cost: {}", name, price)
            }
            Tickets::Vip(price, name) => println!("VIP Ticket is for: {}, cost: {}", name, price),
            Tickets::Standard(price) => println!("Standard ticket, cost: {}", price),
        }
    }
}
