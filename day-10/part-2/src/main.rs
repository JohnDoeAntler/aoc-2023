use std::{collections::VecDeque, io};

const MAP_WIDTH: usize = 140;
const MAP_HEIGHT: usize = 140;

struct Coordinate {
    x: usize,
    y: usize,
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
    let stdin = io::stdin();

    let tmp: [[char; MAP_WIDTH]; MAP_HEIGHT] = stdin
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
                if tmp[y][x] == 'S' {
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
    });

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        let normalized = coord.y * MAP_WIDTH + coord.x;

        if visited[normalized] {
            continue;
        }
        visited[normalized] = true;

        if coord.y > 0
            && get_mapping_by_direction(Direction::Top).contains(&tmp[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Bottom).contains(&tmp[coord.y - 1][coord.x])
        {
            if !visited[normalized - MAP_WIDTH] {
                queue.push_back(Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                });
            }
        }

        if coord.y < MAP_HEIGHT - 1
            && get_mapping_by_direction(Direction::Bottom).contains(&tmp[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Top).contains(&tmp[coord.y + 1][coord.x])
        {
            if !visited[normalized + MAP_WIDTH] {
                queue.push_back(Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                });
            }
        }

        if coord.x > 0
            && get_mapping_by_direction(Direction::Left).contains(&tmp[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Right).contains(&tmp[coord.y][coord.x - 1])
        {
            if !visited[normalized - 1] {
                queue.push_back(Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                });
            }
        }

        if coord.x < MAP_WIDTH - 1
            && get_mapping_by_direction(Direction::Right).contains(&tmp[coord.y][coord.x])
            && get_mapping_by_direction(Direction::Left).contains(&tmp[coord.y][coord.x + 1])
        {
            if !visited[normalized + 1] {
                queue.push_back(Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                });
            }
        }
    }

    // replace starting point with a pipe character
    let tmp = {
        let mut tmp = tmp;

        tmp[start_y][start_x] = {
            let is_bottom = start_y < MAP_HEIGHT - 1 && get_mapping_by_direction(Direction::Top).contains(&tmp[start_y + 1][start_x]);
            let is_right = start_x < MAP_WIDTH - 1 && get_mapping_by_direction(Direction::Left).contains(&tmp[start_y][start_x + 1]);

            match (is_bottom, is_right) {
                (false, false) => 'J',
                (false, true) => 'L',
                (true, false) => '7',
                (true, true) => 'F',
            }
        };

        tmp
    };

    let now = Instant::now();

    // count area
    let mut area = 0;

    for i in 0..MAP_HEIGHT {
        let mut is_parsing_border = false;
        let mut initial_parsing_direction = Direction::Top;
        let mut index = 0;

        for j in 0..MAP_WIDTH {
            let normalized = i * MAP_WIDTH + j;

            if visited[normalized] {
                if !is_parsing_border {
                     match tmp[i][j] {
                        'L' => {
                            is_parsing_border = true;
                            initial_parsing_direction = Direction::Top;
                        },
                        'F' => {
                            is_parsing_border = true;
                            initial_parsing_direction = Direction::Bottom;
                        },
                        '|' => {
                            index += 1;
                            initial_parsing_direction = Direction::Top;
                        },
                        _ => (),
                    };
                } else {
                     match tmp[i][j] {
                        '7' => {
                            is_parsing_border = false;
                            match initial_parsing_direction {
                                Direction::Top => index += 1,
                                _ => (),
                            }
                        },
                        'J' => {
                            is_parsing_border = false;
                            match initial_parsing_direction {
                                Direction::Bottom => index += 1,
                                _ => (),
                            }
                        },
                        '|' => {
                            index += 1;
                        },
                        _ => (),
                    };
                }
            } else if index % 2 == 1 {
                area += 1;
            }
        }
    }

    println!("{:?}", now.elapsed());
    println!("area: {}", area);

}
