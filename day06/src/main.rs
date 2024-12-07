use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Hash)]
enum DIRECTIONS {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Eq for DIRECTIONS {}

fn main() {
    let lines: Vec<Vec<char>> = fs::read_to_string("input")
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();

    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut position: (isize, isize) = (-1, -1);
    let mut direction: DIRECTIONS = DIRECTIONS::NORTH;
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }
        for j in 0..lines[i].len() {
            match lines[i][j] {
                '#' => {
                    obstacles.insert((j as isize, i as isize));
                }
                '^' => {
                    position = (j as isize, i as isize);
                }
                _ => {}
            }
        }
    }

    let init_position: (isize, isize) = position;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    while position.0 >= 0
        && position.0 < lines[0].len() as isize
        && position.1 >= 0
        && position.1 < lines.len() as isize
    {
        visited.insert(position);
        match direction {
            DIRECTIONS::NORTH => {
                position.1 -= 1;
                if obstacles.contains(&position) {
                    position.1 += 1;
                    direction = DIRECTIONS::EAST;
                }
            }
            DIRECTIONS::EAST => {
                position.0 += 1;
                if obstacles.contains(&position) {
                    position.0 -= 1;
                    direction = DIRECTIONS::SOUTH;
                }
            }
            DIRECTIONS::SOUTH => {
                position.1 += 1;
                if obstacles.contains(&position) {
                    position.1 -= 1;
                    direction = DIRECTIONS::WEST;
                }
            }
            DIRECTIONS::WEST => {
                position.0 -= 1;
                if obstacles.contains(&position) {
                    position.0 += 1;
                    direction = DIRECTIONS::NORTH;
                }
            }
        }
    }

    let mut res = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let mut obstacles_added = obstacles.clone();
            obstacles_added.insert((j as isize, i as isize));
            let mut visited_dir: HashSet<(isize, isize, DIRECTIONS)> = HashSet::new();

            let mut position = init_position;
            let mut direction = DIRECTIONS::NORTH;

            while position.0 >= 0
                && position.0 < lines[0].len() as isize
                && position.1 >= 0
                && position.1 < lines.len() as isize
            {
                if visited_dir.contains(&(position.0, position.1, direction)) {
                    res += 1;
                    break;
                }

                visited_dir.insert((position.0, position.1, direction));
                match direction {
                    DIRECTIONS::NORTH => {
                        position.1 -= 1;
                        if obstacles_added.contains(&position) {
                            position.1 += 1;
                            direction = DIRECTIONS::EAST;
                        }
                    }
                    DIRECTIONS::EAST => {
                        position.0 += 1;
                        if obstacles_added.contains(&position) {
                            position.0 -= 1;
                            direction = DIRECTIONS::SOUTH;
                        }
                    }
                    DIRECTIONS::SOUTH => {
                        position.1 += 1;
                        if obstacles_added.contains(&position) {
                            position.1 -= 1;
                            direction = DIRECTIONS::WEST;
                        }
                    }
                    DIRECTIONS::WEST => {
                        position.0 -= 1;
                        if obstacles_added.contains(&position) {
                            position.0 += 1;
                            direction = DIRECTIONS::NORTH;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", res);
}
