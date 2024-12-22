use std::cmp::min;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Button {
    delta: Pos,
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Pos,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Machine {
    fn new(button_a: Button, button_b: Button, prize: Pos) -> Machine {
        Machine {
            button_a,
            button_b,
            prize,
        }
    }

    fn solve(&self, pos: Pos, cost: usize) -> usize {
        if pos == self.prize {
            return 0;
        }

        let step_a = if self.dist(pos + self.button_a.delta) < self.dist(pos) {
            Some(pos + self.button_a.delta)
        } else {
            None
        };

        let step_b = if self.dist(pos + self.button_b.delta) < self.dist(pos) {
            Some(pos + self.button_b.delta)
        } else {
            None
        };

        // println!("Current distance: {} ", self.dist(pos));
        // println!("Checking: {:?} and {:?}", step_a, step_b);

        match step_a {
            Some(step_a) => match step_b {
                Some(step_b) => min(self.solve(step_a, cost + 3), self.solve(step_b, cost + 1)),
                _ => self.solve(step_a, cost + 3),
            },
            _ => match step_b {
                Some(step_b) => self.solve(step_b, cost + 1),
                None => cost + 10000000000000,
            },
        }
    }

    fn dist(&self, pos: Pos) -> usize {
        self.prize.x.abs_diff(pos.x) + self.prize.y.abs_diff(pos.y)
    }
}

impl Button {
    fn new(delta: Pos) -> Button {
        Button { delta }
    }
}

impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let machine_strings: Vec<Vec<String>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .chunks(4)
        .map(|s| s.into())
        .collect();

    let machines: Vec<Machine> = machine_strings
        .iter()
        .map(|machine_string| {
            let button_a_str: Vec<&str> = machine_string[0]
                .split_whitespace()
                .map(|s| s.trim_matches(',').trim_matches('X').trim_matches('Y'))
                .collect();
            let button_b_str: Vec<&str> = machine_string[1]
                .split_whitespace()
                .map(|s| s.trim_matches(',').trim_matches('X').trim_matches('Y'))
                .collect();
            let prize_str: Vec<&str> = machine_string[2]
                .split_whitespace()
                .map(|s| {
                    s.trim_matches(',')
                        .trim_matches('X')
                        .trim_matches('Y')
                        .trim_matches('=')
                })
                .collect();

            let button_a = Button::new(Pos::new(
                button_a_str[2].parse::<isize>().expect("Parsing error"),
                button_a_str[3].parse::<isize>().expect("Parsing error"),
            ));
            let button_b = Button::new(Pos::new(
                button_b_str[2].parse::<isize>().expect("Parsing error"),
                button_b_str[3].parse::<isize>().expect("Parsing error"),
            ));
            let prize = Pos::new(
                prize_str[1].parse::<isize>().expect("Parsing error"),
                prize_str[2].parse::<isize>().expect("Parsing error"),
            );

            Machine::new(button_a, button_b, prize)
        })
        .collect();

    for machine in machines {
        println!("Solve with cost {}", machine.solve(Pos::new(0, 0), 0));
    }
}
