
// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}


// tuple struct
struct TColor(u8, u8, u8);


struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // construct person
    fn new(first: &str, last:&str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn get_tupled_name(self) -> (String, String){
        (self.last_name, self.first_name)
    }
}
pub fn run() {

    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut new_color = TColor(255, 230, 228);

    println!("Color: {} {} {}", new_color.0, new_color.1, new_color.2);

    let mut p = Person::new("Kirill", "Makarov");
    p.set_last_name("Makarovito");
    println!("Person {:?}", p.get_tupled_name());
}