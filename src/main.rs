use std::io;

fn main() {
    loop {
        let mut buffer: Vec<i32> = read_buffer();
        if buffer[0] == 0 && buffer[1] == 0 {
            break;
        }else {
            buffer.sort();
            println!("{} {}", buffer[0], buffer[1]);
        }
    }
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