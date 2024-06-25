use std::io;

#[derive(Debug)]
struct Number {
    from: usize,
    to: usize,
    value: u32,
}

#[derive(Debug)]
struct Part {
    col: usize,
    char: char,
}

fn extract_nums_and_parts () -> (Vec<Vec<Number>>, Vec<Vec<Part>>) {
    let stdin = io::stdin();

    let mut nums: Vec<Vec<Number>> = Vec::new();
    let mut parts: Vec<Vec<Part>> = Vec::new();

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
        let mut current_part_row: Vec<Part> = Vec::new(); 

        for j in 0.. {
            let c = match chars.next() {
                Some(c) => c,
                None => break,
            };

            if c.is_digit(10) {
                last_value = last_value * 10 + c.to_digit(10).unwrap();
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
                current_part_row.push(Part {
                    col: j,
                    char: c,
                });
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

    for i in 0..parts.len() {
        let part_row = &parts[i];
        let rows = get_adjancent_numbers(&nums, i);

        for j in 0..part_row.len() {
            let part = &part_row[j];

            if part.char != '*' {
                continue;
            }

            let mut operands = [None; 2];
            let mut operand_idx = 0;

            for current_num_row in rows.iter() {
                for current_num in current_num_row.iter() {
                    if part.col > current_num.to {
                        continue;
                    }

                    if current_num.from > part.col + 1 {
                        break;
                    }

                    operands[operand_idx] = Some(current_num);
                    operand_idx += 1;
                }
            }

            if operand_idx != 2 {
                continue;
            }

            let operand1 = operands[0].unwrap();
            let operand2 = operands[1].unwrap();

            sum += operand1.value * operand2.value;
        }
    }

    println!("{}", sum);
}

