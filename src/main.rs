use std::io::*;
use std::str::FromStr;

struct Scanner<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }

    fn safe_read<T: FromStr>(&mut self) -> Option<T> {
        let token = self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    loop {
        let h: u32 = sc.read();
        let w: u32 = sc.read();
        if h == 0 { break }
        for i in 0..h {
            for j in 0..w {
                let c = if (i + j) % 2 == 0 {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }
}


