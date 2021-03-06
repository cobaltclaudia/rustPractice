use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[0].clone();
    let name = "Claudia";
    let status = "100%";

    //println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    }else if command == "status" {
        print!("Status is {}", status);
    }else{
        println!("That's not a valid command!");
    }
}