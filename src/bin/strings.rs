// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

fn run() {
    // Immutable fixed-length string type
    let hello = "hello";

    // if we want a string to be String = Growable, heap-allocated data structure
    let mut hello_two = String::from("hello ");

    // Get length of a string
    println!("{}", hello.len());

    // Using the type String we can push characters
    // .push() only works for one character.
    hello_two.push('W');

    // For more use .push_str()
    hello_two.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello_two.capacity());

    // Check if String is empty
    println!("Is Empty: {}", hello_two.is_empty());

    // Check is contains substring
    println!("Contains 'world' {}", hello_two.contains("World"));

    // Replace(from, to), this is case sensitive
    println!("Replace: {}", hello_two.replace("World", "There"));

    // loop through string by whitespace
    for word in hello_two.split_whitespace() {
        println!("Looping: {}", word)
    }

    // Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, hello_two));

    println!("{}", s)
}

fn main() {
    run()
}
