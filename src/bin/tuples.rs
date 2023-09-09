// A tuple is a collection of values of different types. Tuples are constructed using parenthesis ()
// and each tuple itself is a value with type signature

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0); // with numbers.0 I get access to the index 0 of the tuple
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);
    let (employee, access) = ("jake", Access::Full);
    println!("{:?}", access)
}

#[derive(Debug)]
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
