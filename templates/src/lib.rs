mod utils {
    fn read_file() {
        let args: Vec<String> = std::env::args().collect();
        let lines: Vec<Vec<char>> = std::fs::read_to_string(&args[1])
            .expect("Error reading file")
            .lines()
            .map(String::from)
            .map(|s| s.chars().collect())
            .collect();
    }
}

mod matrix {
    use std::collections::VecDeque;

    struct Matrix<T> {
        ds: Vec<VecDeque<T>>,
    }
}