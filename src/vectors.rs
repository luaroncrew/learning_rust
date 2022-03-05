// vectors are resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 2230;

    // add on to vector
    numbers.push(5);
    numbers.pop();
    numbers.push(123);

    println!("{:?}", numbers);

    // get single val
    println!("single value {}", numbers[0]);

    // get vector length
    println!("vector length: {}", numbers.len());

    // vectors are stack allocated
    println!("vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number : {}", x)
    }

    // loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
   println!("Numbers vec : {:?}", numbers)

}