use std::io;
use cached::proc_macro::cached;
use cached::SizedCache;

#[cached(
    type = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(100000) }",
    convert = r#"{ format!("{:?}{:?}", chars.iter().collect::<String>(), nums) }"#
)]
fn solve(chars: &[char], nums: &[usize]) -> u64 {
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

    let sum: u64 = stdin
        .lines()
        .map(|e| e.unwrap())
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
        })
        .map(|e| {
            let mut repeated = String::new();
            for _ in 0..5 {
                repeated.push_str(e[0].as_str());
                repeated.push('?');
            }
            // chars
            let chars = repeated.chars().collect::<Vec<_>>();
            // nums
            let nums = e[1]
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            solve(&chars[..], &nums[..])
        })
        .sum();

    println!("{}", sum);
}
