// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language
fn run() {
    let name = "brad";
    // if we want to mutate a variable we have to specify the keyword mut after let
    // we'll have a warning saying that the initial value assigned to `age` is never read
    // but if we print before we mutate the variable we won't have that warning.
    let mut age = 37;
    println!("My name is: {} and I am {}", name, age);
    // we mutate de data
    age = 38;
    println!("My name is: {} and I am {}", name, age);

    // Define constant
    // If we want to use const de define a variable we'll have to use CAPS and specifiy a type in this case is the type integer 32
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assing multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);

    let _two = 2;
    let _hello = "hello";
    let _j = 's';
    let _my_half: f32 = 0.5;
    let mut _my_bool: bool = true;
}

fn main() {
    run()
}
