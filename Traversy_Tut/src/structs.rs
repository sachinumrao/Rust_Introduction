// used to reate custom datatypers

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color2(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)

    }

    // modify value
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut col = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    col.red = 200;


    println!("Color: {} {} {}", col.red, col.green, col.blue);

    let mut c2 = Color2(255, 0, 0);
    c2.0 = 200;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    //
    let mut p = Person::new("John", "Wick");
    println!("Person {} {}", p.first_name, p.last_name);


    p.set_last_name("Wijard");
    println!("Person {}", p.full_name());

    println!("Person {:?}", p.to_tuple());




}