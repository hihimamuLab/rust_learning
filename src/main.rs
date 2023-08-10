use std::io;

fn main() {
    loop {
        let mut formula_str: String = String::new();
        io::stdin()
            .read_line(&mut formula_str)
            .expect("Error");
        let formula: Vec<&str> = formula_str
            .split_whitespace()
            .collect();
        let num_1: i32 = formula[0].parse().unwrap();
        let num_2: i32 = formula[2].parse().unwrap();
        if formula[1] == "+" {
            println!("{}", num_1 + num_2);
        } else if formula[1] == "-" {
            println!("{}", num_1 - num_2);
        } else if formula[1] == "*" {
            println!("{}", num_1 * num_2);
        } else if formula[1] == "/" {
            println!("{}", num_1 / num_2);
        } else if formula[1] == "?" {
            break;
        } 
    }
}
