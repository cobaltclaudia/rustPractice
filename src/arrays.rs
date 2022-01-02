use std::mem;

pub fn run(){
    let mut numbers: [i32;4] = [1, 2, 3, 4];

    // reassign a value
    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("Single value {}", numbers[2]);
    println!("Array Length {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // reassign a value
    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("Single value {}", numbers[2]);
    println!("Vector Length {}", numbers.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice)
}