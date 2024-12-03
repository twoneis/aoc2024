use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Error reading file");
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut res = 0;

    for line in lines {
        for i in 8..line.len() {
            let pattern = "mul(";
            if line[i - 8..i - 4] != *pattern {
                continue;
            }

            let mut ip = i - 4;
            let mut j = 0;

            let mut num1: Vec<char> = Vec::new();
            while (line.as_bytes()[ip + j] as char).is_digit(10) {
                num1.push(line.as_bytes()[ip + j] as char);
                j += 1;
                if j > 3 {
                    break;
                }
            }

            if num1.len() == 0 {
                continue;
            }

            if (line.as_bytes()[ip + j] as char) != ',' {
                continue;
            }

            ip += j + 1;
            j = 0;
            let mut num2: Vec<char> = Vec::new();
            while (line.as_bytes()[ip + j] as char).is_digit(10) {
                num2.push(line.as_bytes()[ip + j] as char);
                j += 1;
                if j > 3 {
                    break;
                }
            }

            if num2.len() == 0 {
                continue;
            }

            if (line.as_bytes()[ip + j] as char) != ')' {
                continue;
            }

            let a = num1
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .expect("Parsing error");
            let b = num2
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .expect("Parsing error");

            res += a * b;
        }
    }

    println!("Part 1: {res}");
}
