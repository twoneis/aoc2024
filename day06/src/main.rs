use multimap::MultiMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum DIRECTIONS {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn main() {
    let lines: Vec<Vec<char>> = fs::read_to_string("testinput")
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();

    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut position: (isize, isize) = (-1, -1);
    let mut direction: DIRECTIONS = DIRECTIONS::NORTH;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut turns: MultiMap<(isize, isize), DIRECTIONS> = MultiMap::new();

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
                    turns.insert(position, direction);
                }
            }
            DIRECTIONS::EAST => {
                position.0 += 1;
                if obstacles.contains(&position) {
                    position.0 -= 1;
                    direction = DIRECTIONS::SOUTH;
                    turns.insert(position, direction);
                }
            }
            DIRECTIONS::SOUTH => {
                position.1 += 1;
                if obstacles.contains(&position) {
                    position.1 -= 1;
                    direction = DIRECTIONS::WEST;
                    turns.insert(position, direction);
                }
            }
            DIRECTIONS::WEST => {
                position.0 -= 1;
                if obstacles.contains(&position) {
                    position.0 += 1;
                    direction = DIRECTIONS::NORTH;
                    turns.insert(position, direction);
                }
            }
        }
    }

    let mut res = 0;

    for (pos, dirs) in turns.iter_all() {
        for dir in dirs {
            let target = match dir {
                DIRECTIONS::NORTH => (pos.1, DIRECTIONS::SOUTH),
                DIRECTIONS::EAST => (pos.0, DIRECTIONS::WEST),
                DIRECTIONS::SOUTH => (pos.1, DIRECTIONS::NORTH),
                DIRECTIONS::WEST => (pos.0, DIRECTIONS::EAST),
            };

            let mut position = *pos;
            let mut direction = *dir;
            let mut added = false;

            while position.0 >= 0
                && position.0 < lines[0].len() as isize
                && position.1 >= 0
                && position.1 < lines.len() as isize
            {
                if direction == target.1
                    && (position.0 == target.0 || position.1 == target.0)
                    && !added
                {
                    match direction {
                        DIRECTIONS::NORTH => {
                            if !position.1 - 1 < 0 {
                                direction = DIRECTIONS::EAST;
                            }
                        }
                        DIRECTIONS::EAST => {
                            if !position.0 + 1 >= lines[0].len() as isize {
                                direction = DIRECTIONS::SOUTH;
                            }
                        }
                        DIRECTIONS::SOUTH => {
                            if !position.1 + 1 >= lines.len() as isize {
                                direction = DIRECTIONS::WEST;
                            }
                        }
                        DIRECTIONS::WEST => {
                            if !position.0 - 1 < 0 {
                                direction = DIRECTIONS::NORTH;
                            }
                        }
                    };
                    println!("Additional turn at {:?}", position);
                    added = true;
                }

                if position == *pos && added {
                    println!("Valid result");
                    // res += 1;
                }

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
        }
    }

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", res);
}
