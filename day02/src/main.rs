use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Error reading file");

    let lines = content.split("\n");

    let mut res = 0;

    for line in lines {
        let mut order = true;
        let parts: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().expect("Parse error"))
            .collect();
        if parts.len() == 0 {
            continue;
        }

        if parts[0] <= parts[1] {
            for i in 1..parts.len() {
                if parts[i - 1] >= parts[i] || parts[i - 1] + 3 < parts[i] {
                    order = false;
                    break;
                }
            }
        } else {
            for i in 1..parts.len() {
                if parts[i - 1] <= parts[i] || parts[i - 1] > parts[i] + 3 {
                    order = false;
                    break;
                }
            }
        }

        if order {
            res += 1;
        }
    }

    println!("Part 1: {res}");
}
