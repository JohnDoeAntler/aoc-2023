use std::io;

fn solve_horizontal_reflection(pattern: &Vec<Vec<char>>) -> usize {
    let mut offset = 0;
    let mut i = 0;
    let mut j = 1;
    let mut errors = 0;

    loop {
        errors += pattern[i]
            .iter()
            .zip(pattern[j].iter())
            .filter(|(a, b)| a != b)
            .count();

        if errors <= 1 {
            if i == 0 || j == pattern.len() - 1 {
                if errors == 1 {
                    return offset + 1;
                } else {
                    errors = 2;
                    continue;
                }
            }

            i -= 1;
            j += 1;
        } else {
            offset += 1;
            i = offset;
            j = offset + 1;
            errors = 0;

            if j == pattern.len() {
                return 0;
            }
        }
    }
}

fn solve_vertical_reflection(pattern: &Vec<Vec<char>>) -> usize {
    let mut offset = 0;
    let mut i = 0;
    let mut j = 1;
    let mut errors = 0;

    loop {
        errors += pattern
            .iter()
            .map(|e| e[i])
            .zip(pattern.iter().map(|e| e[j]))
            .filter(|(a, b)| a != b)
            .count();

        if errors <= 1 {
            if i == 0 || j == pattern[0].len() - 1 {
                if errors == 1 {
                    return offset + 1;
                } else {
                    errors = 2;
                    continue;
                }
            }

            i -= 1;
            j += 1;
        } else {
            offset += 1;
            i = offset;
            j = offset + 1;
            errors = 0;

            if j == pattern[0].len() {
                return 0;
            }
        }
    }
}

fn solve(pattern: Vec<Vec<char>>) -> usize {
    let intermedidate = solve_horizontal_reflection(&pattern);

    if intermedidate != 0 {
        intermedidate * 100
    } else {
        solve_vertical_reflection(&pattern)
    }
}

fn main() {
    let stdin = io::stdin();

    let mut sum = 0;

    'outer: loop {
        let mut pattern = Vec::new();

        loop {
            let mut input = String::new();

            match stdin.read_line(&mut input) {
                // EOF
                Ok(0) => {
                    if pattern.len() > 0 {
                        break;
                    } else {
                        break 'outer;
                    }
                }
                // empty newline
                Ok(1) => break,
                // normal input
                Ok(_) => {
                    pattern.push(input.trim().chars().collect::<Vec<_>>());
                }
                Err(_) => break,
            }
        }

        sum += solve(pattern);
    }

    println!("{}", sum);
}

