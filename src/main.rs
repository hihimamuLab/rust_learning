use std::io;
use std::io::BufRead;

fn main() {
    let mut arr = vec![[[0; 10]; 3]; 4];
    let stdin = io::stdin();
    for line in stdin.lock().lines().skip(1) {
        let v: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (b, f, r, v) = (v[0] - 1, v[1] - 1, v[2] - 1, v[3]);
        arr[b as usize][f as usize][r as usize] += v;
    }
    for (i, b) in arr.iter().enumerate() {
        for f in b.iter() {
            for r in f.iter() {
                print!(" {}", *r);
            }
            println!("");
        }
        if i < 3 {
            println!("####################");
        }
    }
}


