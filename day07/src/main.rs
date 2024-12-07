fn main() {
    let args: Vec<String> = std::env::args().collect();
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
        .into_iter()
        .filter_map(|line| {
            if solvable(line[0], line[1], &line[2..]) {
                Some(line[0])
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {res}");
}

fn solvable(target: u64, so_far: u64, remaining: &[u64]) -> bool {
    if remaining.len() == 0 {
        target == so_far
    } else {
        solvable(target, so_far + remaining[0], &remaining[1..])
            || solvable(target, so_far * remaining[0], &remaining[1..])
    }
}
