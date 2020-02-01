pub fn run(){
    greeting("Hello", "Jake");

    let get_sum = add(5, 7);
    println!("SUM: {}", get_sum);

    // Closure -> resolves out pf scope issues for variables
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1+n2+n3;
    println!("C SUM: {}", add_nums(3, 5));
}   

fn greeting(greet: &str, name: &str) {
    println!("{} {}, Welcome to the party!", greet, name );
}

fn add(n1: i32, n2: i32) -> i32 {
    // Dont use semi-colon  to return value
    n1+n2
}