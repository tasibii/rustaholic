use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("Slice_____________________: {:?}", slice);
    println!("Length of the slice_______: {}", slice.len());
}

fn main() {
    // fixed-length array
    println!("\n*** Example 1 ***");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array_____________________: {:?}", arr);
    println!("First element of the array: {}", arr[0]);
    println!("Last element of the array_: {}", arr[arr.len() - 1]);
    println!("Length of the array_______: {}", arr.len());
    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&arr));

    println!("\n*** Example 2 ***");
    // Initialize arrays with the same value
    let arrays: [i32; 50] = [0; 50]; // [initial value; length of array]
    println!("Array_____________________: {:?}", arrays);
    println!("First element of the array: {}", arrays[0]);
    println!("Last element of the array_: {}", arrays[arrays.len() - 1]);
    println!("Length of the array_______: {}", arrays.len());
    println!("Array occupies {} bytes", mem::size_of_val(&arrays));


    println!("\n*** Borrow the whole array as a slice ***");
    analyze_slice(&arr);

    println!("\n*** Borrow a section of the array as a slice ***");
    analyze_slice(&arrays[0..15]);

    println!("\n*** Empty ***");
    let empty_array: [i32; 0] = [];
    println!("Array_____________________: {:?}", empty_array);
    println!("Length of the array_______: {}", empty_array.len());
    println!("Array occupies {} bytes", mem::size_of_val(&empty_array));
    analyze_slice(&empty_array);

    // loop over the whole array
    println!("\n*** Loop ***");
    for i in 0..arr.len() + 1 { // Oops, one element too far!
        match arr.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}