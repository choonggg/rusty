pub fn run() {
    let name = "Brad";
    let age = 37;

    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assignin multi
    let (name, age) = ("Chong", 22);
    println!("My name is {} and I am {}", name, age);
}
