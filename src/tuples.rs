// tuples group elements of different types
// max 12 elements
pub fn run(){
    let person: (&str, &str, i8) = ("kirill", "coder", 18);


    println!("{} is {} and he's {} YO", person.0, person.1, person.2);

}