// arrays - fixes list where elements are the same data type

use std::mem;

pub fn run()
{
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // get single value
    println!("{}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array takes up {} bytes", mem::size_of_val(&numbers));

    // get slice of array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}