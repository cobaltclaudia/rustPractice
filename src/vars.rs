pub fn vars_funct(){
    let name = "Claudia";

    let mut count = 0;
    println!("My name is {} and count is at {}", name, count);
    count = 1;
    println!("My name is {} and count is at {}", name, count);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, cookie) = ("Claudia", "oreos");
    println!("{} likes {}", my_name, cookie);

}