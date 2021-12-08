pub fn print_funct(){
    println!("what up! print.rs says hi!");

    println!(
        "{} is from {}",
        "Claudia", "Texas");

    println!(
        "{0} is from {1} and {0} loves {2}",
        "Claudia", "Texas", "Circa Survive");

    println!(
        "{name} likes to {activity}",
        name = "Claudia",
        activity = "go to live shows");

    println!("Binary: {:b} Hex: {:x} Octal: {:o} ", 10, 10, 10);

    println!("{:?}", (12, true, "hello"));

    println!("235434 + 6453 = {}", 235434 + 6453);
}