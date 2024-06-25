use std::io;

#[derive(Debug)]
struct Number {
    from: usize,
    to: usize,
    value: i32,
}

fn extract_nums_and_parts () -> (Vec<Vec<Number>>, Vec<Vec<usize>>) {
    let stdin = io::stdin();

    let mut nums: Vec<Vec<Number>> = Vec::new();
    let mut parts: Vec<Vec<usize>> = Vec::new();

    // extract the numbers and parts from the text
    loop {
        let mut input = String::new();

        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        }

        let mut chars = input.chars();
        let mut last_value = 0;
        let mut length = 0;

        let mut current_num_row: Vec<Number> = Vec::new(); 
        let mut current_part_row: Vec<usize> = Vec::new(); 

        for j in 0.. {
            let c = match chars.next() {
                Some(c) => c,
                None => break,
            };

            if c.is_digit(10) {
                last_value = last_value * 10 + c.to_digit(10).unwrap() as i32;
                length += 1;
                continue;
            }

            if last_value != 0 {
                current_num_row.push(Number {
                    from: j - length,
                    to: j,
                    value: last_value,
                });

                // reset the memory
                last_value = 0;
                length = 0;
            }

            if c != '.' && c != '\n' {
                current_part_row.push(j);
            }
        }

        nums.push(current_num_row);
        parts.push(current_part_row);
    }

    (nums, parts)
}

fn get_adjancent_numbers<T>(e: &Vec<T>, idx: usize) -> Vec<&T> {
    let mut ret = vec![
        &e[idx]
    ];

    if idx > 0 {
        ret.push(&e[idx - 1]);
    }

    if idx < e.len() - 1 {
        ret.push(&e[idx + 1]);
    }

    ret
}

fn main() {
    let (nums, parts) = extract_nums_and_parts();
    let mut sum = 0;

    for i in 0..nums.len() {
        let current = &nums[i];
        let part_rows = get_adjancent_numbers(&parts, i);

        for current_num in current.into_iter() {
            let mut flag = false;

            'outer: for current_part_row in part_rows.iter() {
                for current_part in current_part_row.iter() {
                    if current_num.from > *current_part + 1 {
                        continue;
                    }

                    if *current_part > current_num.to {
                        break;
                    }

                    flag = true;
                    break 'outer;
                }
            }

            if flag {
                sum += current_num.value;
            }
        }
    }

    println!("{}", sum);
}
