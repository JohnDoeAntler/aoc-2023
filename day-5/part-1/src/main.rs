use std::io;

fn get_seeds(stdin: &io::Stdin) -> Vec<u64> {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    stdin.read_line(&mut line).unwrap();

    let colon = line.chars().position(|e| e == ':').unwrap();

    return line[colon + 1..].trim().split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
}

fn main() {
    let stdin = io::stdin();

    let mut seeds = get_seeds(&stdin);

    for _ in 0..7 {
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break
        }

        let mut visited = vec![false; seeds.len()];

        // switching mapping
        loop {
            let mut line = String::new();

            match stdin.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => break
            }

            if line.trim().is_empty() {
                break;
            }

            let arr = line.split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
            let offset = arr[0];
            let from = arr[1];
            let to = arr[1] + arr[2];

            for (i, seed) in seeds.iter_mut().enumerate() {
                if visited[i] {
                    continue;
                }

                if *seed >= from && *seed < to {
                    *seed = offset + (*seed - from);
                    visited[i] = true;
                }
            }
        }
    }

    println!("{:?}", seeds.iter().min().unwrap());
}
