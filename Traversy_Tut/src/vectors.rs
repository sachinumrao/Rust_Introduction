use std::mem;

pub fn run(){
    // fixed group of same kind of values
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    // indexing with [] brackets
    println!("{}", numbers[0]);

    // lenght of array
    println!("Length: {}", numbers.len());

    // amount of memory
    // println!("Memory Consumed: {}", std::mem::size_of_val(&numbers));
    println!("Memory Consumed: {}", mem::size_of_val(&numbers));

    // add element
    numbers.push(10);
    numbers.push(11);

    println!("{:?}", numbers);

    // pop last value
    numbers.pop();
    println!("{:?}", numbers);

    // for loop
    for num in numbers.iter(){
        println!("{}", num);
    }

    // loop and mutate
    for x in numbers.iter_mut(){
        *x *= 3;
    }

    println!("{:?}", numbers);
}