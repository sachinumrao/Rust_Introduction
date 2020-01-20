use std::mem;

pub fn run(){
    // fixed group of same kind of values
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // indexing with [] brackets
    println!("{}", numbers[0]);

    // lenght of array
    println!("Length: {}", numbers.len());

    // amount of memory
    // println!("Memory Consumed: {}", std::mem::size_of_val(&numbers));
    println!("Memory Consumed: {}", mem::size_of_val(&numbers));

    let mut slice: &[i32] = &numbers;
    println!("{:?}", slice);

    let mut slice1: &[i32] = &numbers[0..2];
    println!("{:?}", slice1);


}