use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", nums);

    println!("single val {}", nums[0]);
    println!("length of vector: {}", nums.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));

    println!("Adding to vector");
    nums.push(123);
    nums.push(321);
    println!("Vector {:?}", nums);
    nums.pop();
    println!("Vector {:?}", nums);

    let slice: &[i32] = &nums[0..2];

    println!("Slice of {:?}", slice);

    for x in nums.iter() {
        println!("Number: {}", x)
    }


    for x in nums.iter_mut() {
        *x += 2
    }
    println!("Nums: {:?}", nums);
}
