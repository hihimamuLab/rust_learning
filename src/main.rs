fn main() {
    let mut s1: String = String::from("foo");
    let s2: &str = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
