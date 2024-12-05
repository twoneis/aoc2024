use multimap::MultiMap;
use std::fs;

fn main() {
    let lines: Vec<String> = fs::read_to_string("input")
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let rules: MultiMap<u32, u32> = lines
        .iter()
        .map(|line| {
            line.split("|")
                .filter_map(|s| match s.parse::<u32>() {
                    Ok(r) => Some(r),
                    Err(_) => None,
                })
                .collect::<Vec<u32>>()
        })
        .filter(|line| line.len() == 2)
        .map(|line| (line[0], line[1]))
        .collect();

    let res: u32 = lines
        .iter()
        .map(|line| {
            line.split(",")
                .filter_map(|s| match s.parse::<u32>() {
                    Ok(r) => Some(r),
                    Err(_) => None,
                })
                .collect::<Vec<u32>>()
        })
        .filter(|line| line.len() != 0)
        .filter(|line| {
            for i in 0..line.len() {
                match rules.get_vec(&line[i]) {
                    Some(r) => {
                        for j in 0..i {
                            if r.contains(&line[j]) {
                                return false;
                            }
                        }
                    }
                    None => continue,
                }
            }
            true
        })
        .map(|line| line[line.len() / 2])
        .sum();

    println!("{res}");
}
