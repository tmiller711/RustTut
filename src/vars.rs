// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run()
{
    let name = "Brad";
    let mut age = 17;
    age = 18;
    println!("My name is {}, and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // asign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {} years old", my_name, my_age);
}