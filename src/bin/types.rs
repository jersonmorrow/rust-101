/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they can take in memory)
Floats: f32, f64
Booleat: (bool)
Characater: (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types
// of all variables at compile time, however, the compiler can usually infer what
// type we want to use based on the value and how we use it.

fn run() {
    // Default is "i32" type
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // add explicit type
    let _t: i64 = 21387213723;

    // Find max size
    // this way we get the the max for each type. and we can know which type is better for our needs
    // std is standard library
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // the conventional for variables in rust is snake_case
    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // we specify the type char with single coloms ''
    let a1 = 'a';

    // char type accept emojis as value
    let face = '\u{1F600}';

    // tuples & arrays
    let mut _tup = (1, true, 's');
    let mut _arr_of_num: [i32; 4] = [1, 2, 3, 4];

    // vectors
    // vector is a collection of items of the same type;
    let _item: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", (x, y, _t, is_active, is_greater, a1, face))
}

fn main() {
    run()
}
