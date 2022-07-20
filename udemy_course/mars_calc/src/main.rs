use std::io;


fn main() { // always run first
    let a = 5; // this one is in Stack so no problem here
    let b = a; // b will not be a pointer to a, but will have 5 assigned to it in a memory


    let mut input = String::new();
    some_fn(input);
    io::stdin().read_line(&mut input); // moving ownership to s parameter here.
    // as this string new value is on the Heap and was wrapped by a smart pointer,
    // it was destroyed once the some_fn finished the execution.
    // after that "input" is not pointing to any value anymore
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!(
        "Your weight on Mars is: {} kg", mars_weight);

    // >>> cargo expand to see what is actually happening inside
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}

fn some_fn(str: String) {

}


// ownership rules in Rust

// each value in rust is owned by a variable
// when the owner goes out of scope, the value will be deallocated
// there can only be ONE owner at a time