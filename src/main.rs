use std::io;

fn main() {
    loop {
        let size: Vec<i32> = read_buffer();
        if size == [0, 0] {
            break;
        }else {
            draw_rectangle(size);     
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

fn draw_rectangle(size_vec: Vec<i32>) {
    let grid: Vec<Vec<&str>> = vec![vec!["#"; size_vec[1].try_into().unwrap()]; size_vec[0].try_into().unwrap()];
    
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
