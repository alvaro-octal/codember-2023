use std::fs;

fn main() {
    let contents: String = fs::read_to_string("./data/message_02.txt")
        .expect("Should have been able to read the file").trim().to_lowercase();

    let mut value: i64 = 0;

    for char in contents.chars() {
        if char == '#' {
            value = value + 1
        } else if char == '@'  {
            value = value - 1
        } else if char == '*'  {
            value = value.pow(2)
        } else if char == '&' {
            print!("{value}", value = value)
        } else {
            println!("Invalid character: {char}", char = char);
            return;
        }
    }
}
