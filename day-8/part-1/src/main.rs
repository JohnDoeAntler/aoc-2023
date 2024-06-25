use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let stdin = io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input).unwrap();
    let instructions = input.trim().chars();

    let mut map: HashMap<String, Node> = HashMap::new();

    stdin.lines()
        .map(|e| e.unwrap())
        .for_each(|e| {
            let mut args = e.trim().split(" = ").map(|e| e.to_string());
            let key = args.next().unwrap();
            let value = args.next().unwrap();
            let value = &value[1..value.len()-1];
            let mut args = value.split(", ");
            let left = args.next().unwrap().to_string();
            let right = args.next().unwrap().to_string();

            map.insert(key, Node { left, right });
        });

    let steps = {
        let mut current = "AAA";
        let mut steps: Option<usize> = None;
        let count = instructions.to_owned().count();

        'outer: for i in 0.. {
            for (j, e) in instructions.to_owned().enumerate() {
                if current == "ZZZ" {
                    steps = Some(i * count + j);
                    break 'outer;
                }

                let node = map.get(current).unwrap();

                if e == 'L' {
                    current = &node.left;
                } else {
                    current = &node.right;
                }
            }
        }

        steps.unwrap()
    };

    println!("{:?}", now.elapsed());
    println!("{}", steps);
}
