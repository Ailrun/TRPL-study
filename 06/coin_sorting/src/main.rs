#![allow(dead_code)]
enum Coin0 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u32 {
    use Coin::*;

    match coin {
        Penny => {
            println!("Lucky penny!");
            1
        },
        Nickel => 5,
        Dime => 10,
        Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
