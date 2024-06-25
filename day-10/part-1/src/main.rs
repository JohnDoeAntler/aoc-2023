use std::{collections::VecDeque, io};

const MAP_WIDTH: usize = 140;
const MAP_HEIGHT: usize = 140;

struct Coordinate {
    x: usize,
    y: usize,
    value: usize,
}

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn get_mapping_by_direction(direction: Direction) -> [char; 4] {
    match direction {
        Direction::Top => ['S', 'L', '|', 'J'],
        Direction::Right => ['L', 'S', 'F', '-'],
        Direction::Bottom => ['|', 'F', 'S', '7'],
        Direction::Left => ['J', '-', '7', 'S'],
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let stdin = io::stdin();

    let k: [[char; MAP_WIDTH]; MAP_HEIGHT] = stdin
        .lines()
        .map(|e| e.unwrap())
        .map(|e| e.trim().chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let (start_x, start_y) = {
        let mut start_x = 0;
        let mut start_y = 0;

        'outer: for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if k[y][x] == 'S' {
                    start_x = x;
                    start_y = y;
                    break 'outer;
                }
            }
        }

        (start_x, start_y)
    };

    let mut queue = VecDeque::new();
    let mut visited = [false; MAP_WIDTH * MAP_HEIGHT];

    queue.push_back(Coordinate {
        x: start_x,
        y: start_y,
        value: 0,
    });

    let mut ret = 0;

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        let normalized = coord.y * MAP_WIDTH + coord.x;

        if visited[normalized] {
            continue;
        }
        visited[normalized] = true;

        if coord.y > 0
            && get_mapping_by_direction(Direction::Top).contains(&k[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Bottom).contains(&k[coord.y - 1][coord.x])
        {
            if visited[normalized - MAP_WIDTH] {
                ret = std::cmp::max(coord.value, ret);
            } else {
                queue.push_back(Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                    value: coord.value + 1,
                });
            }
        }

        if coord.y < MAP_HEIGHT - 1
            && get_mapping_by_direction(Direction::Bottom).contains(&k[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Top).contains(&k[coord.y + 1][coord.x])
        {
            if visited[normalized + MAP_WIDTH] {
                ret = std::cmp::max(coord.value, ret);
            } else {
                queue.push_back(Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                    value: coord.value + 1,
                });
            }
        }

        if coord.x > 0
            && get_mapping_by_direction(Direction::Left).contains(&k[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Right).contains(&k[coord.y][coord.x - 1])
        {
            if visited[normalized - 1] {
                ret = std::cmp::max(coord.value, ret);
            } else {
                queue.push_back(Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                    value: coord.value + 1,
                });
            }
        }

        if coord.x < MAP_WIDTH - 1
            && get_mapping_by_direction(Direction::Right).contains(&k[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Left).contains(&k[coord.y][coord.x + 1])
        {
            if visited[normalized + 1] {
                ret = std::cmp::max(coord.value, ret);
            } else {
                queue.push_back(Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                    value: coord.value + 1,
                });
            }
        }
    }

    println!("{:?}", now.elapsed());
    println!("{}", ret);
}
