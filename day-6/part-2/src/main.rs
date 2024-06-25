use std::{io, iter::zip};

fn main() {
    use std::time::Instant;

    let now = Instant::now();

    let stdin = io::stdin();

    let mut time = String::new();
    let mut distance = String::new();

    stdin.read_line(&mut time).expect("Failed to read line");
    stdin.read_line(&mut distance).expect("Failed to read line");

    let time = &time[time.chars().position(|e| e == ':').unwrap()+1..];
    let distance = &distance[distance.chars().position(|e| e == ':').unwrap()+1..];

    let time = time.trim().split_whitespace().collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let distance = distance.trim().split_whitespace().collect::<Vec<_>>().join("").parse::<u64>().unwrap();

    // - x^2 + time * x - distance = 0

    let a = -1;
    let b = time as i64;
    let c = -(distance as i64);

    let delta = b*b - 4*a*c;
    let x1 = (-b as f64 - (delta as f64).sqrt()) / 2.0;
    let x2 = (-b as f64 + (delta as f64).sqrt()) / 2.0;

    let ret = x2 - x1;

    println!("{:?}", now.elapsed());
    println!("{}", ret);
}

