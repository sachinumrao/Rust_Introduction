fn array_sum(){
    // create array
    const ARR_SIZE:u32 = 10;
    let arr:[i32; ARR_SIZE as usize] = [1,2,3,4,5,6,7,8,9,10];

    // run loop to sum array
    let mut arr_sum:i32 = 0;

    for idx in 0..arr.len(){
        arr_sum += arr[idx];
    }


    // print array and sum of array
    println!("Array: {:?}", arr);
    println!("Array Sum: {}", arr_sum);

}


fn main(){
    array_sum();
}