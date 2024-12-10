use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let map: Vec<Vec<u8>> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Parsing error") as u8)
                .collect()
        })
        .collect();

    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                res2 += paths(&map, (i as isize, j as isize), (i as isize - 1, j as isize))
                    + paths(&map, (i as isize, j as isize), (i as isize + 1, j as isize))
                    + paths(&map, (i as isize, j as isize), (i as isize, j as isize - 1))
                    + paths(&map, (i as isize, j as isize), (i as isize, j as isize + 1));

                let mut targets: HashSet<(isize, isize)> = HashSet::new();

                path_target(
                    &map,
                    (i as isize, j as isize),
                    (i as isize - 1, j as isize),
                    &mut targets,
                );
                path_target(
                    &map,
                    (i as isize, j as isize),
                    (i as isize + 1, j as isize),
                    &mut targets,
                );
                path_target(
                    &map,
                    (i as isize, j as isize),
                    (i as isize, j as isize - 1),
                    &mut targets,
                );
                path_target(
                    &map,
                    (i as isize, j as isize),
                    (i as isize, j as isize + 1),
                    &mut targets,
                );

                res1 += targets.len();
            }
        }
    }

    println!("Part 1: {res1}");
    println!("Part 2: {res2}");
}

fn paths(map: &Vec<Vec<u8>>, from: (isize, isize), to: (isize, isize)) -> usize {
    if to.0 < 0
        || to.0 >= map.len() as isize
        || to.1 < 0
        || to.1 >= map[to.0 as usize].len() as isize
    {
        return 0;
    }
    if map[to.0 as usize][to.1 as usize] == map[from.0 as usize][from.1 as usize] + 1 {
        if map[to.0 as usize][to.1 as usize] == 9 {
            1
        } else {
            paths(map, to, (to.0 - 1, to.1))
                + paths(map, to, (to.0 + 1, to.1))
                + paths(map, to, (to.0, to.1 - 1))
                + paths(map, to, (to.0, to.1 + 1))
        }
    } else {
        0
    }
}

fn path_target(
    map: &Vec<Vec<u8>>,
    from: (isize, isize),
    to: (isize, isize),
    targets: &mut HashSet<(isize, isize)>,
) {
    if to.0 < 0
        || to.0 >= map.len() as isize
        || to.1 < 0
        || to.1 >= map[to.0 as usize].len() as isize
    {
        return;
    }
    if map[to.0 as usize][to.1 as usize] == map[from.0 as usize][from.1 as usize] + 1 {
        if map[to.0 as usize][to.1 as usize] == 9 {
            targets.insert(to);
        } else {
            path_target(map, to, (to.0 - 1, to.1), targets);
            path_target(map, to, (to.0 + 1, to.1), targets);
            path_target(map, to, (to.0, to.1 - 1), targets);
            path_target(map, to, (to.0, to.1 + 1), targets);
        }
    }
}
