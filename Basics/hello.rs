use std::io;


fn say_hello(){

    println!("Enter Your Name: ");
    let mut person_name = String::new();

    io::stdin().read_line(&mut person_name).expect("some error occurred...");

    println!("Hello {} !!", person_name);
}

fn main(){
    say_hello();
}