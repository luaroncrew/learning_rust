
pub fn run() {
    let age = 18;
    let check_id = true;
    let knows_person_of_age = true;

    // && - and
    // || - or

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("bartender: drink?");
    } else if age < 21 && check_id {
        println!("sorry, you have to leave");
    } else {
        println!("i need to see your ID")
    }

    // shortand if
    let is_of_age = if age >= 21 { true } else { false };

    println!("is of age: {}", is_of_age);

}