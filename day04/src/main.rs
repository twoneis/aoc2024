use std::collections::VecDeque;
use std::fs;

fn main() {
    let lines: Vec<String> = fs::read_to_string("input")
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut res = 0;

    let m: Vec<VecDeque<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<VecDeque<char>>())
        .filter(|line| line.len() != 0)
        .collect();
    res += count_occurences(&m);

    let mlt = transpose(&shift_left(&m));
    res += count_occurences(&mlt);

    let mt = transpose(&m);
    res += count_occurences(&mt);

    let mrt = transpose(&shift_right(&m));
    res += count_occurences(&mrt);

    let mv = flip_vert(&m);
    res += count_occurences(&mv);

    let mlht = transpose(&flip_horz(&shift_left(&m)));
    res += count_occurences(&mlht);

    let mht = transpose(&flip_horz(&m));
    res += count_occurences(&mht);

    let mrht = transpose(&flip_horz(&shift_right(&m)));
    res += count_occurences(&mrht);

    println!("Part 1: {res}");
}

fn count_occurences(mat: &Vec<VecDeque<char>>) -> u32 {
    let mut res = 0;
    for row in mat {
        for i in 3..row.len() {
            if (row[i - 3] == 'X') && (row[i - 2] == 'M') && (row[i - 1] == 'A') && (row[i] == 'S')
            {
                res += 1;
            }
        }
    }
    res
}

fn transpose(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = vec![VecDeque::new(); mat[0].len()];

    for row in mat {
        let mut i: usize = 0;
        for val in row {
            res[i].push_back(*val);
            i += 1;
        }
    }

    res
}

fn flip_vert(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = vec![VecDeque::new(); mat.len()];

    for i in 0..mat.len() {
        for val in &mat[i] {
            res[i].push_front(*val);
        }
    }

    res
}

fn flip_horz(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = vec![VecDeque::new(); mat.len()];

    for i in 0..mat.len() {
        res[i] = mat[mat.len() - i - 1].clone();
    }

    res
}

fn shift_right(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = mat.clone();

    for i in 0..res.len() {
        for _ in 0..i {
            res[i].push_front('.');
        }
        for _ in i..res.len() {
            res[i].push_back('.');
        }
    }

    res
}

fn shift_left(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = mat.clone();

    for i in 0..res.len() {
        for _ in 0..i {
            res[i].push_back('.');
        }
        for _ in i..res.len() {
            res[i].push_front('.');
        }
    }

    res
}
