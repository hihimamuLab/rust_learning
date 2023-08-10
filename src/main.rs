use std::io;

fn main() {
    let numbers: Vec<i32> = read_buffer();
    let mut factor: i32 = 0;
    for i in numbers[0]..=numbers[1] {
        if numbers[2] % i == 0 {
            factor += 1;   
        }
    }
    println!("{}", factor);
} 

fn read_buffer() -> Vec<i32> {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error");
    buffer.trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Error"))
        .collect()
}
