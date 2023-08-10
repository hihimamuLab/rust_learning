use std::io;

fn main() {
    let number: Vec<i32> = read_buffer();
    let pie: f64 = 3.141592653589793;
    let area: f64 = pie * number[0] as f64 * number[0] as f64;
    let circumference: f64 = pie * number[0] as f64 * 2.0;
    println!("{} {}", area, circumference);
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
