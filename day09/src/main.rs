use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let line: Vec<isize> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Error") as isize)
                .collect::<Vec<isize>>()
        })
        .flatten()
        .collect();

    let mut disk: Vec<isize> = Vec::new();
    let mut skip = false;
    let mut n = -1;

    for i in 0..line.len() {
        let c = if skip {
            -1
        } else {
            n += 1;
            n
        };

        for _ in 0..line[i] {
            disk.push(c);
        }

        skip = !skip;
    }

    let mut i = 0;
    let mut j = disk.len() - 1;

    loop {
        while disk[i] != -1 {
            i += 1;
            if i >= j {
                break;
            }
        }
        while disk[j] == -1 {
            j -= 1;
            if i >= j {
                break;
            }
        }
        if i >= j {
            break;
        }
        disk.swap(i, j);
    }

    let mut sum: usize = 0;

    for block in 0..disk.len() {
        if disk[block] != -1 {
            sum += block * disk[block] as usize;
        }
    }

    println!("Part 1: {}", sum);
}
