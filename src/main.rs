fn main() {
    let list1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    println!("index: {:?}", algorithms::binary_search::<i32>(&list1, &28)); // index: Some(27), iterations: 3
    println!("index: {:?}", algorithms::binary_search::<i32>(&list1, &45)); // index: None, iterations: 5
}
