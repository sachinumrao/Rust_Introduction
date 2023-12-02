fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut inputs = include_str!("input1.txt");

    // println!("{}", inputs);

    // print_type_of(inputs);

    let mut counter: i32 = 0;

    let mut d1: char;
    let mut d2: char;

    for line in inputs.split("\n") {
        let mut digit1: String = String::new();

        for c in line.chars() {
            if c.is_numeric() {
                digit1.push(c);
            }
        }

        if digit1.len() == 1 {
            println!("Single length digit observed");
            digit1 = digit1.repeat(2);
        }

        println!("{}", line);
        println!("digits in line: = {}", digit1);

        d1 = digit1.chars().next().unwrap();
        d2 = digit1.chars().last().unwrap();

        println!("First Digit: {}", d1);
        println!("Second Digit: {}", d2);

        let mut num_string: String = String::new();
        num_string.push(d1);
        num_string.push(d2);

        println!("Required Number: {}", num_string);
        print_type_of(&num_string);

        let mut num: i32 = num_string.parse::<i32>().unwrap();
        // match num {
        //     Ok(value) => println!("parsed integer: {}", value),
        //     Err(err) => println!("parsing error: {}", err),
        // }

        counter += num;
    }

    println!("Final Answer: {}", counter);
}
