use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn get_steps (origin: String, instructions: &Vec<char>, map: &HashMap<String, Node>) -> usize {
    let mut current = origin;
    let count = instructions.to_owned().len();

    for i in 0.. {
        for (j, e) in instructions.to_owned().iter().enumerate() {
            if current.ends_with('Z') {
                return i * count + j;
            }

            let node = map.get(&current).unwrap();

            if *e == 'L' {
                current = node.left.to_string();
            } else {
                current = node.right.to_string();
            }
        }
    }

    0
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let stdin = io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();
    let instructions = input.trim().chars().collect::<Vec<_>>();

    let mut map: HashMap<String, Node> = HashMap::new();

    stdin.lines()
        .map(|e| e.unwrap())
        .for_each(|e| {
            let mut args = e.trim().split(" = ").map(|e| e.to_string());
            let key = args.next().unwrap();
            // remove the parenthesises
            let value = args.next().unwrap();
            let value = &value[1..value.len()-1];
            // the left right values
            let mut args = value.split(", ");
            let left = args.next().unwrap().to_string();
            let right = args.next().unwrap().to_string();

            map.insert(key, Node { left, right });
        });

    let steps = map.keys()
        .to_owned()
        .filter(|e| e.ends_with('A'))
        .map(|e| get_steps(e.to_string(), &instructions, &map))
        .map(|e| e as u64)
        .reduce(|a, b| (a * b) / gcd(a, b))
        .unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    println!("{:?}", steps);
}
