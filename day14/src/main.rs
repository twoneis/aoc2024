use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        println!("Usage: cargo run <input_file> <x_max> <y_max> <iterations>");
        return;
    }

    let x_max = args[2].parse::<isize>().expect("Parse error");
    let y_max = args[3].parse::<isize>().expect("Parse error");

    let iterations = args[4].parse::<isize>().expect("Parse error");

    let rex = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)")
        .expect("Regex error");

    let robots: Vec<((isize, isize), (isize, isize))> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .filter_map(|line| {
            let Some(caps) = rex.captures(&line) else {
                println!("Invalid line: {line}");
                return None;
            };

            let pos: (isize, isize) = (
                (&caps["px"]).parse().expect("Parsing error"),
                (&caps["py"]).parse().expect("Parsing error"),
            );
            let vel: (isize, isize) = (
                (&caps["vx"]).parse().expect("Parsing error"),
                (&caps["vy"]).parse().expect("Parsing error"),
            );
            Some((pos, vel))
        })
        .collect();

    let mut quadrants: [usize; 4] = [0; 4];

    for robot in &robots {
        let pos = robot.0;
        let vel = robot.1;

        let mut x = (pos.0 + iterations * vel.0) % x_max;
        if x < 0 {
            x = x_max + x;
        }
        let mut y = (pos.1 + iterations * vel.1) % y_max;
        if y < 0 {
            y = y_max + y;
        }

        if x < x_max / 2 && y < y_max / 2 {
            quadrants[0] += 1;
        } else if x > x_max / 2 && y < y_max / 2 {
            quadrants[1] += 1;
        } else if x > x_max / 2 && y > y_max / 2 {
            quadrants[2] += 1;
        } else if x < x_max / 2 && y > y_max / 2 {
            quadrants[3] += 1;
        }
    }

    let res = quadrants.iter().fold(1, |acc, x| acc * x);

    println!("With map of size {x_max} {y_max} after {iterations}s: {res}");

    let mut file = File::create("out.txt").expect("File creation error");

    for i in 0..10000 {
        let mut map = vec![vec![0; x_max as usize]; y_max as usize];
        for robot in &robots {
            let pos = robot.0;
            let vel = robot.1;

            let mut x = (pos.0 + i * vel.0) % x_max;
            if x < 0 {
                x = x_max + x;
            }
            let mut y = (pos.1 + i * vel.1) % y_max;
            if y < 0 {
                y = y_max + y;
            }
            map[y as usize][x as usize] += 1;
        }

        file.write_all(b"Iteration ").expect("Write error");
        file.write_all(i.to_string().as_bytes())
            .expect("Write error");
        file.write_all(b"\n").expect("Write error");
        for line in &map[..map.len()] {
            for char in line {
                if *char == 0 {
                    file.write_all(b" ").expect("Write error");
                } else {
                    file.write_all(b"a").expect("Write error");
                }
            }
            file.write_all(b"\n").expect("Write error");
        }
    }
}
