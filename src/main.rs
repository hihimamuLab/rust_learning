use std::{io, collections::HashMap};

const SUITS: &str = "SHCD";

fn main() {
    let mut cards = HashMap::new();

    let mut buf1 = String::new();
    io::stdin()
        .read_line(&mut buf1)
        .expect("error");
    let n: usize = buf1.trim().parse().unwrap();

    for _ in 0..n {
        let mut buf2 = String::new();
        io::stdin()
            .read_line(&mut buf2)
            .expect("error");
        let card: String = buf2.trim().to_string();
        cards.insert(card, true);
    }

    for suit in SUITS.chars() {
        for rank in 1..=13 {
            let card = format!("{} {}", suit, rank);
            if cards.get(&card) == None {
                println!("{}", card);
            }
        }
    }
}
