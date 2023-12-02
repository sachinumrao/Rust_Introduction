use std::collections::HashMap;

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut inputs = include_str!("test_input_2.txt");

    // println!("{}", inputs);

    // print_type_of(inputs);

    let digit_list: [String; 9] = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let mut num_dict: HashMap<String, u32> = HashMap::new();
    for (index, val) in digit_list.iter().enumerate() {
        num_dict.insert(val.clone(), (index + 1) as u32);
    }

    let mut counter: i32 = 0;

    let mut d1: char;
    let mut d2: char;

    for line in inputs.split("\n") {
        let mut digit1: String = String::new();

        for (curr_idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                digit1.push(c);
            } else {
                for str_digit in &digit_list {
                    let mut end_idx = curr_idx + str_digit.len();
                    // get slice of line from curr_idx to end_idx
                    if end_idx < line.len() {
                        let line_substr = &line[curr_idx..end_idx];
                        if line_substr.eq(str_digit) {
                            let a: u32 = num_dict.get(str_digit).unwrap().clone();
                            digit1.push(std::char::from_u32(a).unwrap());
                        }
                    }
                }
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
