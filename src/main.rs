use std::io;

fn main() {
    let numbers: Vec<i32> = read_buffer();
    let division_int: i32 = numbers[0] / numbers[1];
    let surplus: i32 = numbers[0] % numbers[1];
    let division_float: f64 = numbers[0] as f64 / numbers[1] as f64;
    println!("{} {} {}", division_int, surplus, division_float);
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