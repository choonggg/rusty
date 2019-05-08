use std::mem;

pub fn run() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", nums);

    println!("single val {}", nums[0]);
    println!("length of ary: {}", nums.len());
    println!("Array occupies {} bytes", mem::size_of_val(&nums));

    let slice: &[i32] = &nums[0..2];

    println!("Slice of {:?}", slice);
}
