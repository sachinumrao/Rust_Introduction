use std::io;

fn sum_numbers(num1:i32, num2:i32) -> i32{
    return num1 + num2;
}

fn main(){

    let mut num_str1 = String::new();
    let mut num_str2 = String::new();

    println!("Enter First Number: ");
    io::stdin()
        .read_line(&mut num_str1)
        .expect("some error occurred during input...");

    println!("Enter Second Number: ");
    io::stdin()
        .read_line(&mut num_str2)
        .expect("some error occurred during input...");

    let num1:i32 = num_str1
        .trim()
        .parse()
        .expect("some error occurred during parsing...");

    let num2:i32 = num_str2
        .trim()
        .parse()
        .expect("some error occurred during parsing...");

    let numsum = sum_numbers(num1, num2);

    println!("Sum: {}", numsum);
}