// variables hold primitive data or references to data
// variables are immutable(unchangeable) by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Kirill";
    let mut age = 18;

    println!("My name is {} and i am {}", name, age);

    age = 19;

    println!("My name is {} and i am {}", name, age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Kirill", 18);
    println!("{} - {}", my_name, my_age)
}
