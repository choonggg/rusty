pub fn run() {
    // Primitive String
    let hello = "Hello";
    // Growable string
    let mut special_str = String::from("Helo");

    println!("{:?}", hello.len());

    special_str.push_str(" World");

    println!("{:?}", special_str);
    println!("Capacity in Bytes: {}", special_str.capacity());

    // Split
    let harlo = "What is this ??";
    for word in harlo.split_whitespace() {
        println!("{}", word);
    }
    // println!("{:?}", harlo.split_whitespace());

    // String with capazity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
}
