// match is like a switch in JS
// prefer match over if else when working with a single variable
// match considers all posibilities
fn main() {
    operators_with_match(3);
    match_with_booleans(false);
    match_with_numbers(2)
}

fn operators_with_match(value: i32) {
    match value {
        10 => println!("hello world"),
        _ => println!("something else"),
    }
}

fn match_with_booleans(value: bool) {
    match value {
        false => println!("its false"),
        true => println!("it's true "),
    }
}

fn match_with_numbers(value: i8) {
    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("its something else {}", value),
    }
}
