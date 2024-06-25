use std::io;

const WINNING_NUMBER_LENGTH: usize = 10;
const GUESSING_NUMBER_LENGTH: usize = 25;
const LENGTH_OF_WEIGHTS: usize = 203;

// const WINNING_NUMBER_LENGTH: usize = 5;
// const GUESSING_NUMBER_LENGTH: usize = 8;
// const LENGTH_OF_WEIGHTS: usize = 6;

fn main() {
    let stdin = io::stdin();
    let mut weights: [u64; LENGTH_OF_WEIGHTS] = [1; LENGTH_OF_WEIGHTS];

    for i in 0..LENGTH_OF_WEIGHTS {
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

        let mut index = 1;

        for guessing_number in guessing_numbers.iter() {
            for winning_number in winning_numbers.iter() {
                if guessing_number == winning_number {
                    weights[i + index] += weights[i];
                    index += 1;
                }
            }
        }
    }

    let mut sum = 0;

    for i in weights.iter() {
        sum += *i;
    }

    println!("{}", sum);
}

