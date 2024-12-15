use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Cord {
    x: usize,
    y: usize,
}

impl Cord {
    fn new(x: usize, y: usize) -> Cord {
        Cord { x, y }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let lines: Vec<Vec<char>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .filter_map(|s| match s.len() {
            0 => None,
            _ => Some(s.chars().collect()),
        })
        .collect();

    let map: Vec<Vec<char>> = lines
        .iter()
        .filter_map(|line| match line[0] {
            '#' => Some(line.to_owned()),
            _ => None,
        })
        .collect();

    let steps: Vec<char> = lines
        .iter()
        .filter_map(|line| match line[0] {
            '#' => None,
            _ => Some(line.to_owned()),
        })
        .flatten()
        .collect();

    let mut robot: Cord = Cord::new(0, 0);
    let mut walls: HashSet<Cord> = HashSet::new();
    let mut boxes: HashSet<Cord> = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '#' => {
                    walls.insert(Cord::new(j, i));
                }
                'O' => {
                    boxes.insert(Cord::new(j, i));
                }
                '@' => {
                    robot = Cord::new(j, i);
                }
                _ => {}
            };
        }
    }

    for s in steps {
        if can_step(s, &robot, &walls, &boxes) {
            step(s, &mut robot, &walls, &mut boxes);
        }
    }

    let res1: usize = boxes.iter().map(|cord| 100 * cord.y + cord.x).sum();

    println!("Part 1: {res1}")
}

fn can_step(direction: char, robot: &Cord, walls: &HashSet<Cord>, boxes: &HashSet<Cord>) -> bool {
    let mut cur: Cord = *robot;
    match direction {
        '>' => {
            while {
                cur.x += 1;
                boxes.contains(&cur)
            } {}
        }
        '<' => {
            while {
                cur.x -= 1;
                boxes.contains(&cur)
            } {}
        }
        'v' => {
            while {
                cur.y += 1;
                boxes.contains(&cur)
            } {}
        }
        '^' => {
            while {
                cur.y -= 1;
                boxes.contains(&cur)
            } {}
        }
        _ => panic!("Unexpected direction!"),
    }

    match walls.get(&cur) {
        Some(_) => false,
        _ => true,
    }
}

fn step(direction: char, robot: &mut Cord, walls: &HashSet<Cord>, boxes: &mut HashSet<Cord>) {
    let mut cur: Cord = *robot;
    match direction {
        '>' => {
            robot.x += 1;
            while {
                cur.x += 1;
                boxes.contains(&cur)
            } {}
        }
        '<' => {
            robot.x -= 1;
            while {
                cur.x -= 1;
                boxes.contains(&cur)
            } {}
        }
        'v' => {
            robot.y += 1;
            while {
                cur.y += 1;
                boxes.contains(&cur)
            } {}
        }
        '^' => {
            robot.y -= 1;
            while {
                cur.y -= 1;
                boxes.contains(&cur)
            } {}
        }
        _ => panic!("Unexpected direction!"),
    }

    if boxes.contains(robot) {
        boxes.remove(robot);
        boxes.insert(cur);
    }
}
