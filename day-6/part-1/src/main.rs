use std::{io, iter::zip};

fn main() {
    let stdin = io::stdin();

    let mut times = String::new();
    let mut distances = String::new();

    stdin.read_line(&mut times).expect("Failed to read line");
    stdin.read_line(&mut distances).expect("Failed to read line");

    let times = &times[times.chars().position(|e| e == ':').unwrap()+1..];
    let distances = &distances[distances.chars().position(|e| e == ':').unwrap()+1..];

    let times = times.trim().split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = distances.trim().split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let pairs = zip(times, distances);

    let mut ret = 1;

    for (time, distance) in pairs {
        let left = 1;
        let right = time / 2;

        let k = {
            let mut ret = 0;

            for i in left..=right {
                let j = time - i;

                if i * j > distance {
                    ret = right - i + 1;
                    break;
                }
            }

            ret
        };

        let by_one = if time % 2 == 0 { 1 } else { 0 };

        ret *= k * 2 - by_one;
    }

    println!("{}", ret);
}
