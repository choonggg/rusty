// Standard Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Kolor(u8, u8, u8);

// OOP
struct Person {
    fname: String,
    lname: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            fname: first.to_string(),
            lname: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.fname, self.lname)
    }

    fn set_lname(&mut self, last: &str) -> String {
        let new_lname = last.to_string();
        self.lname = new_lname;
        self.lname.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.fname, self.lname)
    }
}

pub fn run() {
    // Struct
    let mut c = Color{red: 255, green: 0, blue: 0};

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple Struct
    let k = Kolor(0, 255, 255);
    println!("Kolor: {} {} {}", k.0, k.1, k.2);

    let mut p = Person::new("Jon", "Do");

    println!("Person: {} {}", p.fname, p.lname);
    println!("Fullname: {}", p.full_name());
    p.set_lname("Bo");
    println!("New name: {}", p.full_name());
    println!("Tuple: {:?}", p.to_tuple());
}
