use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Hand {
    normalized_value: u64,
    normalized_type: u8,
    bid_amount: u64,
}

const VALUE_MAP: [char; 15] = [
    '_', // placeholder
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'T',
    'J',
    'Q',
    'K',
    'A',
];

fn get_rank(str: String) -> u8 {
    let mut map = HashMap::new();

    str.chars().for_each(|e| {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    });

    map.keys().map(|e| (e, *map.get(e).unwrap())).filter(|(_,v)| *v > 1).map(|(_,v)| v * v).sum()
}

// 4, 8, 9, 13, 16, 25

fn to_normalized_value(str: String) -> u64 {
    println!("{:?}", str);

    str.chars()
        .enumerate()
        .map(|(i, e)| 
             (u64::pow(15, (5 - i) as u32), e))
            .map(|(w, e)| VALUE_MAP.iter().position(|v| *v == e).unwrap() as u64 * w)
        .sum()
}

fn main() {
    let stdin = io::stdin();

    let mut e = stdin.lines()
        .map(|e| e.unwrap())
        .filter(|e| !e.is_empty())
        .map(|e| e.trim().split_whitespace().map(|e| e.to_string()).collect::<Vec<_>>())
        .map(|e| Hand {
            normalized_value: to_normalized_value(e[0].clone()),
            normalized_type: get_rank(e[0].clone()),
            bid_amount: e[1].parse::<u64>().unwrap(),
        })
        .collect::<Vec<_>>();

    e.sort_by(|a, b| {
        if a.normalized_type == b.normalized_type {
            a.normalized_value.cmp(&b.normalized_value)
        } else {
            a.normalized_type.cmp(&b.normalized_type)
        }
    });

    let k = e.iter()
        .enumerate()
        .map(|(i, e)| (i + 1) as u64 * e.bid_amount).sum::<u64>();

    println!("{:?}", k);
}
