pub fn run() {
    // Primitive array 
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    // Non primitve vectors: can directly copy, 
    // we have to set reference by using &
    let vec1 = vec![1,2,3,4];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));



}