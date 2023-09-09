fn main() {
    basic_down_count_loop();
    basic_counting_loop();
    let arr = [10, 20, 30, 40, 50];
    let result = for_in_arrays(&arr);
    println!("{:?}", result)
}

fn basic_down_count_loop() {
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    println!("DONE")
}

fn basic_counting_loop() {
    let mut i = 1;
    loop {
        println!("{:?}", i);
        i = i + 1;
        if i == 5 {
            break;
        }
    }
}

fn for_in_arrays(array: &[i32]) -> &i32 {
    let mut result: &i32 = &0;
    let target: &i32 = &50;
    for (_index, value) in array.iter().enumerate() {
        if value == target {
            result = value;
        }
    }
    return result;
}
