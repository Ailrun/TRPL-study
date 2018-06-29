#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello world");
    let word = first_word0(&s);
    s.clear();

    let s = String::from("hello world");
    let word = first_word1(&s);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");
    let len = s.len();

    let slice = &s[0..2];
    println!("The value of slice is: {}", slice);
    let slice = &s[..2];
    println!("The value of slice is: {}", slice);

    let slice = &s[3..len];
    println!("The value of slice is: {}", slice);
    let slice = &s[3..];
    println!("The value of slice is: {}", slice);

    let slice = &s[..];
    println!("The value of slice is: {}", slice);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    let word = first_word(&my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (ind, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..ind];
        }
    }

    &s[..]
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (ind, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..ind];
        }
    }

    &s[..]
}

fn first_word0(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (ind, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return ind;
        }
    }

    s.len()
}
