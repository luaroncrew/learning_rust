// arrays - fixed list where elements are the same data types

pub fn run(){
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 2230;

    println!("{:?}", numbers);

    // get single val
    println!("single value {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

}