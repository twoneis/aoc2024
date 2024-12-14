use disjoint::DisjointSet;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let lines: Vec<Vec<char>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();

    let mut perimeters: HashMap<usize, usize> =
        HashMap::with_capacity(lines.len() * lines[0].len());
    let mut regions: DisjointSet = DisjointSet::with_len(lines.len() * lines[0].len());

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let id = i * lines.len() + j;

            let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);
            if i != 0 {
                neighbors.push((i - 1, j));
            }
            if j != 0 {
                neighbors.push((i, j - 1));
            }
            if i != lines.len() - 1 {
                neighbors.push((i + 1, j));
            }
            if j != lines[i].len() - 1 {
                neighbors.push((i, j + 1));
            }

            let mut perimeter = 4 - neighbors.len();
            for neighbor in neighbors {
                let neighbor_id = neighbor.0 * lines.len() + neighbor.1;
                if lines[neighbor.0][neighbor.1] == lines[i][j] {
                    regions.join(id, neighbor_id);
                } else {
                    perimeter += 1;
                }
            }

            perimeters.insert(id, perimeter);
        }
    }

    let mut res1 = 0;

    for set in regions.sets() {
        let area = set.len();
        let mut perimeter = 0;
        for id in set {
            match perimeters.get(&id) {
                Some(p) => perimeter += p,
                None => panic!("No perimeter for id"),
            }
        }
        res1 += area * perimeter;
    }

    println!("Part 1: {res1}");
}
