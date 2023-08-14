fn main() {
    let mut buf1: String = String::new();
    std::io::stdin().read_line(&mut buf1).expect("error");
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf2).expect("error");

    let seq_a_rev: Vec<&str> = buf2.split_whitespace().rev().collect();
    let ans = seq_a_rev.join(" ");

    println!("{}", ans);
}
