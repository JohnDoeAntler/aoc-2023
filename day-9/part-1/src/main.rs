use std::io;

fn extrapolate(arr: &Vec<i64>) -> i64 {
    if arr.iter().all(|e| *e == 0) {
        return 0;
    }
    let last_value = arr.last().unwrap();
    let diffs = Vec::from(arr.windows(2).map(|e| e[1] - e[0]).collect::<Vec<_>>());
    let extrapolated_difference = extrapolate(&diffs);

    extrapolated_difference + last_value
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
        .collect::<Vec<_>>();

    println!("{}", ret.iter().sum::<i64>());
}
