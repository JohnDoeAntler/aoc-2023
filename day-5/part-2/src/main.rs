use std::io;

#[derive(Debug, Clone, Copy)]
struct Seed {
    from: u64,
    to: u64,
    stage: u8,
}

fn get_seeds(stdin: &io::Stdin) -> Vec<Seed> {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    stdin.read_line(&mut line).unwrap();

    let colon = line.chars().position(|e| e == ':').unwrap();

    return line[colon + 1..]
        .trim()
        .split_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|args| Seed { from: args[0], to: args[0] + args[1], stage: 0 })
        .collect();
}

fn main() {
    let stdin = io::stdin();
    let mut seeds = get_seeds(&stdin);

    for i in 0..7 {
        let i = i + 1;
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break
        }

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

            let mut arr = line.split_whitespace().map(|e| e.parse::<u64>().unwrap());
            let offset = arr.next().unwrap();
            let from = arr.next().unwrap();
            let to = from + arr.next().unwrap();

            for j in 0..seeds.len() {
                // seeds those will not mapped
                let seed = seeds[j];

                if seed.stage == i || seed.to <= from || seed.from >= to {
                    continue;
                }

                // seeds those will get mapped
                if seed.from < from && from <= seed.to {
                    seeds.push(Seed { from: seed.from, to: from, stage: seed.stage });
                }

                if seed.from <= to && to < seed.to {
                    seeds.push(Seed { from: to, to: seed.to, stage: seed.stage });
                }

                let left = std::cmp::max(seed.from, from) - from + offset;
                let right = std::cmp::min(seed.to, to) - from + offset;
                seeds[j] = Seed { from: left, to: right, stage: i };
            }
        }
    }

    println!("{:?}", seeds.iter().map(|e| e.from).min().unwrap());
}

