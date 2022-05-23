/*
Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or own string data
*/

pub fn run()
{
    // immutable
    let hello = "Hello";
    // muttable
    let mut world = String::from("World ");

    // get length of string
    println!("Length: {}", hello.len());

    // lets you add one character
    world.push('W');

    // lets you add a string
    world.push_str("ordle");

    // capacity in bytes
    println!("Capacity: {}", world.capacity());

    // check is a string is empty
    println!("Is empty: {}", world.is_empty());

    // contains a substring?
    println!("Contains 'Wordle': {}", world.contains("Wordle"));

    // replace
    println!("Replace: {}", world.replace("Wordle", "There"));

    // loop through by white space
    for word in world.split_whitespace()
    {
        println!("{}", word);
    } 

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing. Checks if two things are equal
    // only throws an error if it fails. Else it does nothing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (s));
}