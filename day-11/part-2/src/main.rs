use std::io;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let stdin = io::stdin();

    let map = stdin.lines()
        .map(|e| e.unwrap().trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_row_indices = map.iter()
        .enumerate()
        .filter(|(_, e)| e.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let empty_col_indices = {
        let mut indices = Vec::new();
        for i in 0..map[0].len() {
            if map.iter().all(|e| e[i] == '.') {
                indices.push(i);
            }
        }
        indices
    };

    let gallaxies = map.iter()
        .enumerate()
        .filter(|(i, _)| !empty_row_indices.contains(i))
        .map(|(i, e)| e.iter()
            .enumerate()
            .filter(|(j, _)| !empty_col_indices.contains(j))
            .filter(|(_, e)| **e == '#')
            .map(|(j, _)| Coordinate {
                y: i + empty_row_indices.iter().fold(0, |a, b| if i > *b { a + 1_000_000 - 1 } else { a }),
                x: j + empty_col_indices.iter().fold(0, |a, b| if j > *b { a + 1_000_000 - 1 } else { a }),
            })
            .collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Vec<_>>();

    let mut ret = 0;

    for (i, j) in gallaxies.iter().enumerate() {
        for (k, l) in gallaxies.iter().skip(i + 1).enumerate() {
            if i == k + i + 1 {
                continue;
            }

            let local = (j.x as i32 - l.x as i32).abs() as usize + (j.y as i32 - l.y as i32).abs() as usize;
            ret += local;
        }
    }

    println!("Result: {}", ret);
}

