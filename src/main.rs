#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    println!("{:?}", scores.entry(String::from("Yellow")));
    println!("{:?}", scores.entry(String::from("Blue")));

    println!("{:?}", scores);
}
