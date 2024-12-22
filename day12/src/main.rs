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

    let res1 = part1(&lines);

    let res2 = part2(&lines);

    println!("Part 1: {res1}");
    println!("Part 2: {res2}");
}

fn part2(lines: &Vec<Vec<char>>) -> usize {
    let mut perimeters: HashMap<usize, usize> =
        HashMap::with_capacity(lines.len() * lines[0].len());
    let mut regions: DisjointSet = DisjointSet::with_len(lines.len() * lines[0].len());

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let id = pos_id((i, j), lines.len());

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

    let mut res = 0;

    for set in regions.sets() {
        let area = set.len();
        let mut corners = 0;

        for id in &set {
            let mut neighbors = [[false; 3]; 3];
            let (i, j) = id_pos(*id, lines.len());

            if i != 0 && j != 0 && set.contains(&pos_id((i - 1, j - 1), lines.len())) {
                neighbors[0][0] = true;
            }
            if i != 0 && set.contains(&pos_id((i - 1, j), lines.len())) {
                neighbors[0][1] = true;
            }
            if i != 0 && set.contains(&pos_id((i - 1, j + 1), lines.len())) {
                neighbors[0][2] = true;
            }

            if j != 0 && set.contains(&pos_id((i, j - 1), lines.len())) {
                neighbors[1][0] = true;
            }
            if set.contains(&pos_id((i, j + 1), lines.len())) {
                neighbors[1][2] = true;
            }

            if j != 0 && set.contains(&pos_id((i + 1, j - 1), lines.len())) {
                neighbors[2][0] = true;
            }
            if set.contains(&pos_id((i + 1, j), lines.len())) {
                neighbors[2][1] = true;
            }
            if set.contains(&pos_id((i + 1, j + 1), lines.len())) {
                neighbors[2][2] = true;
            }

            if (neighbors[0][1] && neighbors[1][0] && !neighbors[0][0])
                || (!neighbors[0][1] && !neighbors[1][0])
            {
                corners += 1;
            }
            if (neighbors[0][1] && neighbors[1][2] && !neighbors[0][2])
                || (!neighbors[0][1] && !neighbors[1][2])
            {
                corners += 1;
            }
            if (neighbors[2][1] && neighbors[1][0] && !neighbors[2][0])
                || (!neighbors[2][1] && !neighbors[1][0])
            {
                corners += 1;
            }
            if (neighbors[2][1] && neighbors[1][2] && !neighbors[2][2])
                || (!neighbors[2][1] && !neighbors[1][2])
            {
                corners += 1;
            }
        }

        res += area * corners;
    }

    res
}

fn part1(lines: &Vec<Vec<char>>) -> usize {
    let mut perimeters: HashMap<usize, usize> =
        HashMap::with_capacity(lines.len() * lines[0].len());
    let mut regions: DisjointSet = DisjointSet::with_len(lines.len() * lines[0].len());

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let id = pos_id((i, j), lines.len());

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
                let neighbor_id = pos_id(neighbor, lines.len());
                if lines[neighbor.0][neighbor.1] == lines[i][j] {
                    regions.join(id, neighbor_id);
                } else {
                    perimeter += 1;
                }
            }

            perimeters.insert(id, perimeter);
        }
    }

    let mut res = 0;

    for set in regions.sets() {
        let area = set.len();
        let mut perimeter = 0;
        for id in set {
            match perimeters.get(&id) {
                Some(p) => perimeter += p,
                None => panic!("No perimeter for id"),
            }
        }
        res += area * perimeter;
    }

    res
}

fn pos_id((i, j): (usize, usize), n: usize) -> usize {
    i * n + j
}

fn id_pos(id: usize, n: usize) -> (usize, usize) {
    (id / n, id % n)
}
