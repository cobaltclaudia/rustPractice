use std::mem;

pub fn run(){

    let mut numb:Vec<i32> = vec![1, 2, 3, 4];
    // reassign a value
    numb[2] = 20;

    numb.push(4);
    numb.push(8);
    numb.pop();

    println!("{:?}", numb);
    println!("Single value {}", numb[2]);
    println!("Vector Length {}", numb.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&numb));

    // get slice
    let slice: &[i32] = &numb[1..3];
    println!("Slice: {:?}", slice);

    for x in numb.iter(){
        println!("Number: {} ", x)
    }

    for x in numb.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numb);
}