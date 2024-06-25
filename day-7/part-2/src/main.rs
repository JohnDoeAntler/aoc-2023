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
    'J',
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
    'Q',
    'K',
    'A',
];

fn get_type(str: String) -> u8 {
    let mut map = HashMap::new();

    str.chars().filter(|e| *e != 'J').for_each(|e| {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    });

    let j_count = str.chars().filter(|e| *e == 'J').count();
    let mut ret = map.values().map(|e| *e).collect::<Vec<_>>();

    // base case: five of a kind and all jokers
    if j_count == 5 {
        return 25;
    }

    // otherwise sum the jokers to the highest value
    ret.sort();
    ret.reverse();
    ret[0] += j_count as u8;
    ret.iter().map(|e| e.pow(2)).sum()
}

fn to_normalized_value(str: String) -> u64 {
    str.chars()
        .enumerate()
        .map(|(i, e)| (VALUE_MAP.len().pow((5 - i) as u32), e))
        .map(|(w, e)| VALUE_MAP.iter().position(|v| *v == e).unwrap() as u64 * w as u64)
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
            normalized_type: get_type(e[0].clone()),
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

