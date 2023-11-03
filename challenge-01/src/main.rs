use std::collections::HashMap;
use std::fs;
use std::str::Split;

fn main() {
    let contents: String = fs::read_to_string("./data/message_01.txt")
        .expect("Should have been able to read the file").trim().to_lowercase();

    let mut counters: HashMap<&str, u64> = HashMap::new();

    let words: Split<&str> = contents.split(" ");

    let mut order: Vec<&str> = Vec::new();

    for (_, word) in words.enumerate() {
        if counters.contains_key(word) {
            counters.insert(word, counters[word] + 1);
        } else {
            order.push(word);
            counters.insert(word, 1);
        }
    }

    let mut chain: String = "".to_owned();

    for word in order {
        chain.push_str(&*format!("{word}{counter}", word = word, counter = counters[word]));
    }

    println!("{chain}", chain = chain)
}
