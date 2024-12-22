use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <input_file> <n_iterations>");
        return;
    }

    let iterations: usize = args[2]
        .parse::<usize>()
        .expect("Usage: cargo run <input_file> <n_iterations>");

    let stones: Vec<String> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(|s| s.split_whitespace().map(String::from))
        .flatten()
        .collect();
    let len = stones.len();

    let res: usize = (0..iterations)
        .fold(
            HashMap::from_iter(stones.into_iter().zip(vec![1; len])),
            |acc, _| step(&acc),
        )
        .values()
        .sum();

    println!("After {} iterations: {} stones", iterations, res);
}

fn step(stones: &HashMap<String, usize>) -> HashMap<String, usize> {
    stones.iter().fold(HashMap::new(), |mut acc, (stone, num)| {
        match stone.as_str() {
            "0" => {
                *acc.entry("1".to_string()).or_insert(0) += num;
                acc
            }
            s if s.len() % 2 == 0 => {
                let lower: String = s[..s.len() / 2]
                    .parse::<usize>()
                    .expect("Parsing error")
                    .to_string();
                let upper: String = s[s.len() / 2..]
                    .parse::<usize>()
                    .expect("Parsing error")
                    .to_string();
                *acc.entry(lower.to_string()).or_insert(0) += num;
                *acc.entry(upper.to_string()).or_insert(0) += num;
                acc
            }
            s => {
                *acc.entry((s.parse::<usize>().expect("Parsing error") * 2024).to_string())
                    .or_insert(0) += num;
                acc
            }
        }
    })
}
