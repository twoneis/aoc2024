use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines: Vec<String> = fs::read_to_string()
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect();

    let mut res = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
    }
}
