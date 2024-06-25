use std::io;

const WINNING_NUMBER_LENGTH: usize = 10;
const GUESSING_NUMBER_LENGTH: usize = 25;

fn main() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;

    loop {
        let mut input = String::new();

        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        };

        let colon = input.chars().position(|e| e == ':').unwrap();
        let input = &input[colon + 1..];

        let arr = input.split('|').collect::<Vec<_>>();
        let winning_numbers: [u64; WINNING_NUMBER_LENGTH] = arr[0].trim()
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let guessing_numbers: [u64; GUESSING_NUMBER_LENGTH] = arr[1].trim()
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut exp = 1;

        for guessing_number in guessing_numbers.iter() {
            for winning_number in winning_numbers.iter() {
                if guessing_number == winning_number {
                    exp <<= 1;
                    break;
                }
            }
        }

        sum += exp >> 1;
    }

    println!("{}", sum);
}
