pub fn run () {
    // print to console
    println!("hello from print.rs file");

    // basic formatting
    println!("number is {}", 1);
    println!("{} is from {}", "Kirill", "Russia");

    //positional args
    println!("{0} is from {1} and {0} likes to {2}", "Kirill", "Russia", "code");

    //named arguments
    println!("{name} likes to {activity}", name="Kirill", activity="code");

    //placeholder traits
    println!("Binary {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10)
}