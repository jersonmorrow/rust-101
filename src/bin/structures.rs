// structures are like interfaces in typescript.
// Are intended to define the shape of an element in this case of a structure.

// A struct with two fields

fn main() {
    let name: String = String::from("Peter");
    let age: u8 = 27;
    let _peter: Person = Person { name, age };
    let _my_box: ShippingBox = ShippingBox {
        depth: 10,
        width: 10,
        height: 2,
    };
    let cereal = GroceryItem {
        price: 2.99,
        stock: 10,
    };
    println!("{:?}", cereal.price);

    let drink = Drink {
        flavor: DrinkFlavors::Strawberry,
        fluid_ounce: 2.34,
    };
    get_drink_props(&drink);
    get_drink_flavors(&drink.flavor);
}

struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

struct Person {
    name: String,
    age: u8,
}

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

#[derive(Debug)]
enum DrinkFlavors {
    BubleGum,
    Strawberry,
    Lemonade,
}

struct Drink {
    flavor: DrinkFlavors,
    fluid_ounce: f64,
}

fn get_drink_flavors(flavors: &DrinkFlavors) {
    match flavors {
        DrinkFlavors::BubleGum => println!("{:?}", "BubleGum"),
        DrinkFlavors::Lemonade => println!("{:?}", "Lemonade"),
        DrinkFlavors::Strawberry => println!("{:?}", "Strawberry"),
    }
}

fn get_drink_props(drink: &Drink) {
    println!("{:?}", drink.flavor);
    println!("{}", drink.fluid_ounce)
}
