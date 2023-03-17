#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    println!("Coin::Penny is {}", value_in_cents(Coin::Penny));
    println!("Coin::Nickel is {}", value_in_cents(Coin::Nickel));
    println!("Coin::Dime is {}", value_in_cents(Coin::Dime));
    println!(
        "Coin::Quarter is {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => 1,
        4 => 2,
        a => a,
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
