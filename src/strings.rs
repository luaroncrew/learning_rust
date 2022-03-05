
// Primitive str = Immutable
// fixed-length string somewhere in memory
// String = Growable, heap-allocated
// data structure - Use when you need
// to modify or own string data

pub fn run(){
    // changeable string
    let mut hello = String::from("hello, ");

    // get length
    println!("length: {}", hello.len());

    // push char
    hello.push('W');

    //push string
    hello.push_str("orld");

    // capacity in bytes
    println!("capcity: {}", hello.capacity());

    // check if empty
    println!("empty: {}", hello.is_empty());

    // check if contains substring
    println!("{} contains World? - {}",hello, hello.contains("World"));
    println!("{}", hello);

    // replace
    println!("replace: {}", hello.replace("World", "The"));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len()); // true
    assert_eq!(20, s.capacity()); // false


    println!("{}", s);
}