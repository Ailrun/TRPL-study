#![allow(unused_variables, dead_code)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    use Option::*;

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let absent_string = None::<&str>;
}
