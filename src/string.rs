
pub fn run(){

    let mut circa = String::from("Circa ");

    println!("Length: {}", circa.len());

    circa.push_str("Survive");

    println!("Capacity: {}", circa.capacity());

    println!("Is Empty: {}", circa.is_empty());

    println!("Contains 'Circa' {}", circa.contains("Circa"));
    println!("Contains 'circa' {}", circa.contains("circa"));
    println!("Replace: {}", circa.replace("Survive", "fuckin Survive"));

    for word in circa.split_whitespace(){
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

    println!("{}", circa);
}