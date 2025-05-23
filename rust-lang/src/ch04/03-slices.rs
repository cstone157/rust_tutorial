//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    // You don't need to include the begining or end of the slice
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];


    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];


    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];


    let my_string = String::from("hello world");
    // `first_word3` works on slices of `String`s, whether partial or whole
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word3` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word3(&my_string);
    let my_string_literal = "hello world";
    // `first_word3` works on slices of string literals, whether partial or whole
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);


    // General slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}