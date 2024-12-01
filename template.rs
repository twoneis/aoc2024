use std::fs;

fn main() {
    let lines = fs::read_to_string("testinput")
        .expect("Error reading file")
        .split("\n");

    let mut res = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
    }
}
