use std::env;

pub fn run()
{
    // how to gets command line arguments
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Tristin";
    let status = "100%";

    if command == "hello"
    {
        println!("Hi {}, how are you?", name);
    }
    else if command == "status"
    {
        println!("Status is {}", status);
    }
    else
    {
        println!("That is not a valid argument");
    }

    // println!("Command: {}", command);
}