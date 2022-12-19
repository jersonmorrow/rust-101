pub fn run() {
    // Print to console
    println!("hello from the print.rs");

    // I can't print literal numbers. Rust will read {} in order to print whatever we want to print
    println!("{}", 1);

    // basic formmating
    println!("{} is {}", "brad", "mass");

    // Positional arguments using index
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Jhon",
        activity = "Baseball"
    );

    // Placeholder traits
    // this will provide the Binary number of 10, the Hexagonal and the Octal
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // PlaceHolder for debug trait
    // {:?} will help us to put in on multiple values
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10)
}
