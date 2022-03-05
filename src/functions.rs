
pub fn run() {
    greeting("Hello", "Kirill");

    //bind function values to variables
    let get_sum = add(1, 5);
    println!("sum is {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2:i32| n1+n2+n3;
    println!("c sum {}", add_nums(3, 9));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
