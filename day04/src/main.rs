use std::collections::VecDeque;
use std::fs;

fn main() {
    let lines: Vec<String> = fs::read_to_string("testinput")
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let original: Vec<VecDeque<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<VecDeque<char>>())
        .filter(|line| line.len() != 0)
        .collect();

    let transpose: Vec<VecDeque<char>> = transpose(&original);

    let flip_vert: Vec<VecDeque<char>> = flip_vert(&original);
    let flip_horz: Vec<VecDeque<char>> = flip_horz(&original);

    let shift_right: Vec<VecDeque<char>> = shift_right(&original);
    let shift_left: Vec<VecDeque<char>> = shift_left(&original);

    let mut res = 0;

    println!("Original:");
    for line in &original {
        println!("{:?}", line);
    }

    println!("Transpose:");
    for line in &transpose {
        println!("{:?}", line);
    }

    println!("Flip Vertical:");
    for line in &flip_vert {
        println!("{:?}", line);
    }
    println!("Flip Horizontal:");
    for line in &flip_horz {
        println!("{:?}", line);
    }

    println!("Shift right:");
    for line in &shift_right {
        println!("{:?}", line);
    }
    println!("Shift left:");
    for line in &shift_left {
        println!("{:?}", line);
    }
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
            res[i].pop_back();
            res[i].push_front('.');
        }
    }

    res
}

fn shift_left(mat: &Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    let mut res: Vec<VecDeque<char>> = mat.clone();

    for i in 0..res.len() {
        for _ in 0..i {
            res[i].pop_front();
            res[i].push_back('.');
        }
    }

    res
}
