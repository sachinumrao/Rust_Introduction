pub fn run(){
    // different ypes of values grouped together

    let person: (&str, &str, i8) = ("John", "NYC", 20);
    println!("{} is from {} and is {}.", person.0, person.1, person.2);

    

}