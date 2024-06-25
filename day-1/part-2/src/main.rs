use std::io::{self};

fn main() {
    let stdin = io::stdin();

    let mut sum = 0;

    let numbers = [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    loop {
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        }

        let len = line.len();

        let first_digit_min_map = numbers.into_iter().map(|e| {
            line.find(e).unwrap_or(len)
        }).collect::<Vec<_>>();

        let first_digit_min = first_digit_min_map.iter().position(|e| e == first_digit_min_map.iter().min().unwrap()).unwrap() % 9 + 1;

        let last_digit_max_map = numbers.into_iter().map(|e| {
            match line.rfind(e) {
                Some(v) => v + e.len(),
                None => 0,
            }
        }).collect::<Vec<_>>();

        let last_digit_max = last_digit_max_map.iter().position(|e| e == last_digit_max_map.iter().max().unwrap()).unwrap() % 9 + 1;

        sum += first_digit_min * 10 + last_digit_max;
    }

    println!("{}", sum);
}

