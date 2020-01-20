pub fn run(){

    let fixed_string = "Hello";

    // mutable string
    let mut hello = String::from("Hello ");
    println!("String: {}", hello);
    
    println!("Length: {}", hello.len());

    hello.push('W');

    println!("String: {}", hello);

    hello.push_str("orld!");

    println!("String: {}", hello);

    println!("Capacity: {}", hello.capacity());

    println!("Is_Empty: {}", hello.is_empty());

    // contain substring
    println!("Contains 'orl': {}", hello.contains("orld"));

    println!("Length: {}", hello.len());

    // replace substring
    let mod_str = hello.replace("World", "Universe");

    println!("Replace: {}", hello.replace("World", "Universe"));
    println!("Modified String: {}", mod_str);

    // for loop
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion
    assert_eq!(2, s.len());

    assert_eq!(10, s.capacity());




}