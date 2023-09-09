fn main() {
    let age = 15;
    if age >= 21 {
        println!("ok to purchase")
    } else {
        println!("cannot purchase")
    }

    gretting(true);
    operators(2);
}

fn gretting(value: bool) {
    if value {
        println!("hello")
    } else {
        println!("goodbye")
    }
}

fn operators(value: i32) {
    if value > 5 {
        println!(">5");
        return;
    }
    if value < 5 {
        println!("<5");
        return;
    }
    println!("==5")
}
