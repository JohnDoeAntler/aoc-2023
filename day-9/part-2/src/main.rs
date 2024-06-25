use std::io;

fn extrapolate(arr: &Vec<i64>) -> i64 {
    if arr.iter().all(|e| *e == 0) {
        return 0;
    }
    let first_value = arr.first().unwrap();
    let diffs = Vec::from(arr.windows(2).map(|e| e[1] - e[0]).collect::<Vec<_>>());
    let extrapolated_difference = extrapolate(&diffs);

    first_value - extrapolated_difference
}

fn main() {
    let stdin = io::stdin();

    let ret = stdin
        .lines()
        .map(|e| e.unwrap())
        .map(|e| e.trim()
             .split_whitespace()
             .map(|e| e.parse::<i64>().unwrap())
             .collect::<Vec<_>>()
             .try_into()
             .unwrap()
        )
        .map(|e| extrapolate(&e))
        .sum::<i64>();

    println!("{}", ret);
}

