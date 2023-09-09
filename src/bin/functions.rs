// Topic: Functions
// Program requirements:
// / * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the printin macro to display messages to the terminal

fn print_name() {
    let first_name = "Jerson";
    let last_name = "morrow";

    println!("my name is {}{}", first_name, last_name)
}

fn main() {
    print_name();
}
