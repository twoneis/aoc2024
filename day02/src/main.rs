use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Error reading file");

    let lines = content.split("\n");

    let mut res = 0;
    let mut res2 = 0;

    for line in lines {
        let parts: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().expect("Parse error"))
            .collect();
        if parts.len() == 0 {
            continue;
        }

        if sorted(&parts) {
            res += 1;
        }

        if semi_sorted(&parts) {
            res2 += 1;
        }
    }

    println!("Part 1: {res}");
    println!("Part 1: {res2}");
}

fn sorted(parts: &Vec<u64>) -> bool {
    if parts[0] <= parts[1] {
        for i in 1..parts.len() {
            if parts[i - 1] >= parts[i] || parts[i - 1] + 3 < parts[i] {
                return false;
            }
        }
    } else {
        for i in 1..parts.len() {
            if parts[i - 1] <= parts[i] || parts[i - 1] > parts[i] + 3 {
                return false;
            }
        }
    }

    true
}
fn semi_sorted(parts: &Vec<u64>) -> bool {
    if sorted(parts) {
        true
    } else {
        for i in 0..parts.len() {
            let mut cpy = parts.clone();
            cpy.remove(i);
            if sorted(&cpy) {
                return true;
            }
        }
        false
    }
}
