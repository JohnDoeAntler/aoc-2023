use std::io;

#[derive(Debug)]
struct Set {
    r: u64,
    g: u64,
    b: u64,
}

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

        let idx = line.chars().position(|e| e == ':').unwrap() + 1;
        let line = &line[idx..];

        let mut set = Set {
            r: 0,
            g: 0,
            b: 0,
        };

        for e in line.split(";") {
            let e = e.trim();
            for e in e.split(",") {
                let e = e.trim();
                let args = e.split(" ").collect::<Vec<_>>();

                let number = args[0].parse::<u64>().unwrap();
                match &args[1][..1] {
                    "r" => set.r = if set.r < number { number } else { set.r },
                    "g" => set.g = if set.g < number { number } else { set.g },
                    "b" => set.b = if set.b < number { number } else { set.b },
                    _ => (),
                }
            }
        }

        sum += set.r * set.g * set.b;
    }

    println!("{}", sum);
}

