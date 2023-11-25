use std::fs;
use regex::Regex;

fn main() {
    let contents: String = fs::read_to_string("./data/message_03.txt")
        .expect("Should have been able to read the file").trim().to_lowercase();

    let re = Regex::new(r"(?m)^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let mut counter: u64 = 0;

    for (_, line) in contents.lines().enumerate() {
        let Some(captures) = re.captures(line) else {
            println!("Line does not match regex: {line}", line = line);
            continue;
        };

        let min: u64 = captures[1].parse::<u64>().unwrap();
        let max: u64 = captures[2].parse::<u64>().unwrap();

        let char: char = captures[3].chars().nth(0).unwrap();

        let appearances: u64 = u64::try_from(captures[4].chars().filter(|x| *x == char).count()).unwrap();

        if !(min <= appearances && appearances <= max) {
            counter = counter + 1;

            if counter == 42 {
                println!("Password: {password}", password = captures[4].trim());
                return;
            }
        }
    }
}
