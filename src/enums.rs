// enums are types which have a few definite values

enum Movement
{
    // variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement)
{
    // perform action depending on info
    // match is similiar to switch
    match m
    {
        Movement::Up => println!("Avatar moves up"),
        Movement::Down => println!("Avatar moves down"),
        Movement::Left => println!("Avatar moves left"),
        Movement::Right => println!("Avatar moves right")
    }
}

pub fn run()
{
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}