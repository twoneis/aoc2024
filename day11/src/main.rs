fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let mut stones: Vec<String> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(|s| s.split_whitespace().map(String::from))
        .flatten()
        .collect();

    for _ in 0..25 {
        // println!("{:?}", stones);
        let mut i = 0;
        while i < stones.len() {
            match stones[i].as_ref() {
                "0" => stones[i] = "1".to_string(),
                _ if stones[i].len() % 2 == 0 => {
                    let lower: String = stones[i][..stones[i].len() / 2]
                        .parse::<usize>()
                        .expect("Parsing error")
                        .to_string();
                    let upper: String = stones[i][stones[i].len() / 2..]
                        .parse::<usize>()
                        .expect("Parsing error")
                        .to_string();
                    stones.remove(i);
                    stones.insert(i, lower);
                    i += 1;
                    stones.insert(i, upper);
                }
                _ => {
                    let new: String =
                        (stones[i].parse::<usize>().expect("Parsing error") * 2024).to_string();
                    stones.remove(i);
                    stones.insert(i, new);
                }
            }
            i += 1;
        }
    }

    println!("Part 1: {}", stones.len());
}
