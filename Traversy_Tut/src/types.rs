pub fn run(){
    let x = 1; //default is integer 32
    let y = 1.0; // default is float64

    let z: i64 = 32432434;

    let is_active: bool = true;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("Is_Active: {}", is_active);

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // characters
    let a1 = 'a';
    println!("Char: {}", a1);


}