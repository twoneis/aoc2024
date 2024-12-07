fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let lines: Vec<Vec<u64>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|l| {
            l.split_whitespace()
                .filter(|l| l.len() != 0)
                .map(|s| {
                    s.split(":").collect::<Vec<&str>>()[0]
                        .parse()
                        .expect("Parsing error")
                })
                .collect()
        })
        .collect();

    let res: u64 = lines
        .iter()
        .filter_map(|line| {
            if solvable(line[0], line[1], &line[2..]) {
                Some(line[0])
            } else {
                None
            }
        })
        .sum();

    let res2: u64 = lines
        .iter()
        .filter_map(|line| {
            if solvable2(line[0], line[1], &line[2..]) {
                Some(line[0])
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {res}");
    println!("Part 2: {res2}");
}

fn solvable(target: u64, so_far: u64, remaining: &[u64]) -> bool {
    if remaining.len() == 0 {
        target == so_far
    } else {
        solvable(target, so_far + remaining[0], &remaining[1..])
            || solvable(target, so_far * remaining[0], &remaining[1..])
    }
}

fn solvable2(target: u64, so_far: u64, remaining: &[u64]) -> bool {
    if remaining.len() == 0 {
        target == so_far
    } else {
        solvable2(target, so_far + remaining[0], &remaining[1..])
            || solvable2(target, so_far * remaining[0], &remaining[1..])
            || solvable2(
                target,
                (so_far.to_string() + &remaining[0].to_string())
                    .parse()
                    .expect("Concat error"),
                &remaining[1..],
            )
    }
}
