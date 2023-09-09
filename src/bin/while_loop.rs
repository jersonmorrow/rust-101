fn main() {
    basi_while_loop();
    count_down_while_loop()
}

fn basi_while_loop() {
    let mut index = 1;
    while index <= 3 {
        println!("{:?}", index);
        index = index + 1;
    }
}

fn count_down_while_loop() {
    let mut counter = 5;
    while counter >= 1 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("DONE")
}
