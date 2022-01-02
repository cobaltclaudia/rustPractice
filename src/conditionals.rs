pub fn run(){
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what do you want to drink?")
    }else if age <= 21 && check_id {
        println!("Get the hell outta here");
    }else {
        print!("Bartender: show me your id beeech");
    }

    let is_of_age = if age >=21 {true} else {false};
    print!("Is of age: {}", is_of_age);
}