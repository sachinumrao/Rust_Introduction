extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("We are playing a guessing game...");

    let secret_num = rand::thread_rng().gen_range(1,10);

    println!("Please input your guess : ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You Guessed : {}", guess);
    println!("Secret Number : {}", secret_num);

}
