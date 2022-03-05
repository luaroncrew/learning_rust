/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language,
// which means that it must know the types of
// all variables at compile time, however, the compiler
// can usually infer what type we
// want to use based on the value and how we use it.


pub fn run(){
    // default is i32
    let x = 1;

    //default is f64
    let y = 2.5;

    // add explicit type
    let l: i64 = 2131244123;

    // find max size
    println!("Max i 32 {}", std::i32::MAX);
    println!("Max i 32 {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 > 5;

    // char
    let a1 = 'a';

    // unicode chars
    let face = '\u{1F600}';


    println!("{:?}", (x, y, l, is_active, is_greater, a1, face));

    

}
