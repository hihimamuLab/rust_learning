fn main() {
let mut v: Vec<i32> = Vec::new();

v.push(5);
v.push(6);
v.push(7);

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.22),
    SpreadsheetCell::Text(String::from("blue")),
];

match &row[2] {
    SpreadsheetCell::Int(first) => println!("{}", first),
    SpreadsheetCell::Float(second) => println!("{}", second),
    SpreadsheetCell::Text(third) => println!("{}", third), 
}
}

