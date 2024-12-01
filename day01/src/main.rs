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

    println!("{res}");
}
