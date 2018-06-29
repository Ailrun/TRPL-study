#![allow(unused_variables, dead_code, unused_assignments)]

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
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    };

    use Coin::*;

    let coin = Penny;
    let mut count = 0;
    match coin {
        Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    };

    let coin = Penny;
    let mut count = 0;
    if let Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
