pub fn run() {
    println!("Hello from print.rs file");
    println!("{}", 1);
    println!("{name} likes to {ahem}", name = "John", ahem = "Play");
    println!("Binary {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("Debugging placeholder {:?}", (12, true));
}
