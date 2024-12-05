use multimap::MultiMap;
use std::collections::HashMap;
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

    let res1: u32 = lines
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

    let res2: u32 = lines
        .iter()
        .map(|line| {
            line.split(",")
                .filter_map(|s| match s.parse::<u32>() {
                    Ok(r) => Some(r),
                    Err(_) => None,
                })
                .collect::<Vec<u32>>()
        })
        .filter_map(|line| {
            let mut swapped;
            let mut some_swap = false;
            let mut arr: Vec<u32> = line.clone();
            for i in 0..arr.len() {
                swapped = false;
                for j in 0..line.len() - i - 1 {
                    match rules.get_vec(&arr[j + 1]) {
                        Some(r) => {
                            if r.contains(&arr[j]) {
                                arr.swap(j, j + 1);
                                swapped = true;
                                some_swap = true;
                            }
                        }
                        None => continue,
                    }
                }

                if !swapped {
                    break;
                }
            }
            if some_swap {
                Some(arr)
            } else {
                None
            }
        })
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
    // .filter(|line| line.len() != 0)
    // .collect();

    println!("Part 1: {res1}");
    println!("Part 2: {res2}");
    // for line in res2 {
    //     println!("{:?}", line);
    // }
}
