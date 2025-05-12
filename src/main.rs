//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    //let s = String::from("hello");
    //change(&s);
    let mut s = String::from("hello");
    change2(&mut s);


    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);


    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;


    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);


    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");


    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the value is not dropped.

//fn change(some_string: &String) {
//    some_string.push_str(", world");
//} // Illegal since you can't change a variable that's outside of your scope

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

  fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
