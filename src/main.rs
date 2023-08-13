fn main() {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("error");

    let n = input_string.trim().parse::<usize>().unwrap();
    
    for i in 1..=n {
        if i % 3 == 0 {
            print!(" {}", i);
        } else {
            let mut j = i;
            loop {
                if j % 10 == 3 {
                    print!(" {}", i);
                    break;
                }
                j /= 10;
                if j == 0 { break; }
            }
        }
    }
    println!("");
}