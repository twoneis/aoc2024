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

    let mut antinodes1: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes2: HashSet<(isize, isize)> = HashSet::new();

    for (_, mut locations) in antennas.into_iter() {
        locations.sort();
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let y_step = locations[i].0 - locations[j].0;
                let x_step = locations[i].1 - locations[j].1;
                let mut y = locations[i].0;
                let mut x = locations[i].1;
                while y >= 0 && x >= 0 && y < lines[0].len() as isize && x < lines.len() as isize {
                    antinodes2.insert((x, y));
                    x += x_step;
                    y += y_step;
                }

                let y_step = locations[j].0 - locations[i].0;
                let x_step = locations[j].1 - locations[i].1;
                let mut y = locations[j].0;
                let mut x = locations[j].1;
                while y >= 0 && x >= 0 && y < lines[0].len() as isize && x < lines.len() as isize {
                    antinodes2.insert((x, y));
                    x += x_step;
                    y += y_step;
                }

                let y1 = locations[i].0 + (locations[i].0 - locations[j].0);
                let x1 = locations[i].1 + (locations[i].1 - locations[j].1);

                let y2 = locations[j].0 + (locations[j].0 - locations[i].0);
                let x2 = locations[j].1 + (locations[j].1 - locations[i].1);

                antinodes1.insert((x1, y1));
                antinodes1.insert((x2, y2));
            }
        }
    }

    antinodes1 = antinodes1
        .into_iter()
        .filter(|antinode| {
            antinode.0 >= 0
                && antinode.0 < lines[0].len() as isize
                && antinode.1 >= 0
                && antinode.1 < lines.len() as isize
        })
        .collect();

    antinodes2 = antinodes2
        .into_iter()
        .filter(|antinode| {
            antinode.0 >= 0
                && antinode.0 < lines[0].len() as isize
                && antinode.1 >= 0
                && antinode.1 < lines.len() as isize
        })
        .collect();

    println!("Part 1: {}", antinodes1.len());
    println!("Part 2: {}", antinodes2.len());
}
