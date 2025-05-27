//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

#[derive(Debug)]
enum Coin<'a>  {
    Penny,
    Nickel,
    Dime,
    Quarter(&'a str),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        //Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        Coin::Quarter(String) => println!("State quarter from {coin:?}!"),
        _ => count += 1,
    }
    println!("Count of non-quarter coins {count}.");

    let mut count = 0;
    if let Coin::Quarter("Nevada")= coin {
        println!("State quarter from {coin:?}!");
    } else {
        count += 1;
    }
    println!("Count of non-quarter coins {count}.");
}