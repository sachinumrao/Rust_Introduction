use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone(); // 0 is the path of target

    let name = "Brad";

    println!("Args: {}", command);

    if command == "hello" {
        println!("Hi {}", name);

    }
}