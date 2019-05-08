pub fn run() {
  // Default is i32
  let x = 1;

  // Default is f64
  let y = 2.5;

  // explicit type
  let z: i64 = 424242424242;

  // Find max size
  println!("max i32: {}", std::i32::MAX);
  println!("max i32: {}", std::i64::MAX);

  // Bool
  let is_active = true;

  println!("{:?}", (x, y, z, is_active));
}
