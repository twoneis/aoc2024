use multimap::MultiMap;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let lines: Vec<Vec<char>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .filter(|l| l.len() != 0)
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();

    let mut antennas: MultiMap<char, (isize, isize)> = MultiMap::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] != '.' {
                antennas.insert(lines[y][x], (x as isize, y as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, mut locations) in antennas.into_iter() {
        locations.sort();
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let y1 = locations[i].0 + (locations[i].0 - locations[j].0);
                let x1 = locations[i].1 + (locations[i].1 - locations[j].1);
                let y2 = locations[j].0 + (locations[j].0 - locations[i].0);
                let x2 = locations[j].1 + (locations[j].1 - locations[i].1);

                antinodes.insert((x1, y1));
                antinodes.insert((x2, y2));
            }
        }
    }

    antinodes = antinodes
        .into_iter()
        .filter(|antinode| {
            antinode.0 >= 0
                && antinode.0 < lines[0].len() as isize
                && antinode.1 >= 0
                && antinode.1 < lines.len() as isize
        })
        .collect();

    println!("Part 1: {}", antinodes.len());
}
