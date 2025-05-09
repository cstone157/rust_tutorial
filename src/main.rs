use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

fn main() {
    // let x = 5; // Will result in an error since, we try and change an inmuttable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    let f: bool = false; // with explicit type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple
    let (z1, z2, z3) = tup;
    println!("The value of z1 is: {z1}");
    println!("The value of tup.2 is: {}", tup.2);

    let months = ["January", "February", "March", "April", "May", "June", "July", // Array
              "August", "September", "October", "November", "December"];
    
              let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}