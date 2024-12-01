use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Error reading file");
    let lines = contents.split("\n");

    let mut vec1: Vec<i64> = vec![];
    let mut vec2: Vec<i64> = vec![];

    let mut res: u64 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
        vec1.push(parts[0].parse::<i64>().expect("Unable to parse"));
        vec2.push(parts[1].parse::<i64>().expect("Unable to parse"));
    }

    vec1.sort();
    vec2.sort();

    for i in 0..vec1.len() {
        let mut dif: i64 = vec1[i] - vec2[i];
        if dif < 0 {
            dif = -dif;
        }
        res += dif as u64;
    }

    println!("Part 1: {res}");

    res = 0;

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < vec1.len() {
        let val = vec1[i];

        while j < vec2.len() && vec2[j] < vec1[i] {
            j += 1;
        }

        if j >= vec2.len() {
            break;
        }

        let mut jp = j;
        while vec1[i] == vec2[jp] {
            res += val as u64;
            jp += 1;
        }

        i += 1;
    }

    println!("Part 2: {res}");
}
