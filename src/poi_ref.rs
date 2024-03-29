 fn main() {
    // Primitive types
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // Vector
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));
}
