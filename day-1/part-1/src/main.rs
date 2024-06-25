use std::io::{self};

fn main() {
    let stdin = io::stdin();

    let mut sum: u64 = 0;

    loop {
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        }

        let mut line = line.chars();

        let first_digit = loop {
            match line.next() {
                Some(c) => {
                    if c.is_digit(10) {
                        break c.to_digit(10).unwrap();
                    }
                }
                None => break 0,
            } ;
        };

        let last_digit = loop {
            match line.next_back() {
                Some(c) => {
                    if c.is_digit(10) {
                        break c.to_digit(10).unwrap();
                    }
                }
                None => break first_digit,
            } ;
        };

        sum += (first_digit * 10 + last_digit) as u64;
    }

    println!("{}", sum);
}
