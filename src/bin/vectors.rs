fn run() {
    fn print_out_item(item: &Vec<u32>) {
        for i in item {
            println!("{}", i);
        }
    }

    fn execute() {
        // vector is a collection of items of the same type;
        let item = vec![1, 2, 3];
        print_out_item(&item);
        print_out_item(&item);
    }

    execute();
}

fn main() {
    run()
}
