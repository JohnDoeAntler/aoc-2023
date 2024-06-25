use std::io;

fn solve(chars: &[char], nums: &[usize]) -> u32 {
    if nums.len() == 0 && (chars.len() == 0 || chars.iter().all(|e| *e == '.' || *e == '?')) {
        return 1;
    }

    if chars.len() == 0 || nums.len() == 0 {
        return 0;
    }

    let mut sum = 0;
    let current = nums[0];

    for (idx, window) in chars.windows(current + 1).enumerate() {
        if window.iter().take(current).all(|e| *e == '#' || *e == '?')
            && (window[current] == '.' || window[current] == '?')
        {
            sum += solve(&chars[current + idx + 1..], &nums[1..]);
        }

        if window[0] == '#' {
            break;
        }
    }

    sum
}

fn main() {
    let stdin = io::stdin();

    let sum: u32 = stdin
        .lines()
        .map(|e| e.unwrap())
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
        })
        .map(|e| {
            let mut chars = e[0].chars().collect::<Vec<_>>();
            chars.push('.');
            let nums = e[1]
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            solve(&chars[..], &nums[..])
        })
        .sum();

    println!("{}", sum);
}
