const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;

    println!("Firing {} of my {} missiles", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left.", missiles);

    let (mut var_1, var_2): (i32, i32) = (100, 200);
    println!("{} - {}", var_1, var_2);
    var_1 = STARTING_MISSILES;
    println!("{} - {}", var_1, var_2);
}
