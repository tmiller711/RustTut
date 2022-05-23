// conditionals - used to check the condition of something and act on the result

pub fn run()
{
    let age = 18;
    let check_id = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age
    {
        println!("Bartender: What would you like to drink");
    }
    else if age < 21 && check_id
    {
        println!("Bartender: Sorry you have to be 21");
    } 
    else 
    {
        println!("Bartender: I'll need to see your ID");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}