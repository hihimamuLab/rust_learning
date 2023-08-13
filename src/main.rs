use std::io;
fn main() {
    loop {
        let size: Vec<usize> = read_buffer();
        let (hight, width): (usize, usize) = (size[0], size[1]);
        if hight == 0 && width == 0 { break; }
        let row_str: String = String::from("#").repeat(width) + "\n";
        let line_str: String = format!("{}{}{}\n", "#", String::from(".").repeat(width - 2), "#");
        println!("{}{}{}", row_str, line_str.repeat(hight - 2), row_str);
    }
}

fn read_buffer() -> Vec<usize> {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error");
    buffer.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Error"))
        .collect()
}
