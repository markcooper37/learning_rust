fn main() {
    println!("Value of a penny: {}", value_in_cents(Coin::Penny));
    println!("Value of a nickel: {}", value_in_cents(Coin::Nickel));
    println!("Value of a dime: {}", value_in_cents(Coin::Dime));
    println!("Value of a quarter: {}", value_in_cents(Coin::Quarter));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Function to return a coin's value
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
