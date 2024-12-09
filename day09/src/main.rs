fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let line: Vec<isize> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Error") as isize)
                .collect::<Vec<isize>>()
        })
        .flatten()
        .collect();

    let mut disk: Vec<isize> = Vec::new();
    let mut skip = false;
    let mut n = -1;

    for i in 0..line.len() {
        let c = if skip {
            -1
        } else {
            n += 1;
            n
        };

        for _ in 0..line[i] {
            disk.push(c);
        }

        skip = !skip;
    }

    let mut disk1 = disk.clone();

    let mut i = 0;
    let mut j = disk.len() - 1;

    loop {
        while disk[i] != -1 {
            i += 1;
            if i >= j {
                break;
            }
        }
        while disk[j] == -1 {
            j -= 1;
            if i >= j {
                break;
            }
        }
        if i >= j {
            break;
        }
        disk.swap(i, j);
    }

    let res: usize = checksum(&disk);

    println!("Part 1: {res}");

    let mut i = disk1.len() - 1;

    while i > 0 {
        while i > 0 && disk1[i] == -1 {
            i -= 1;
        }
        let mut n: usize = 0;

        let num = disk1[i] as usize;

        while i > 0 && disk1[i] == num as isize {
            n += 1;
            i -= 1;
        }

        for j in 0..i + 1 {
            if fits(&disk1[j..j + n], n as usize) {
                for pos in 0..n as usize {
                    disk1.swap(i + 1 + pos, j + pos)
                }
                break;
            }
        }
    }

    let res = checksum(&disk1);
    println!("Part 2: {res}");
}

fn fits(part: &[isize], size: usize) -> bool {
    if part.len() < size {
        return false;
    }

    for i in 0..size {
        if part[i] != -1 {
            return false;
        }
    }
    true
}

fn checksum(disk: &[isize]) -> usize {
    let mut sum = 0;
    for block in 0..disk.len() {
        if disk[block] != -1 {
            sum += block * disk[block] as usize;
        }
    }
    sum
}

fn print_disk(disk: &[isize]) {
    for &num in disk {
        if num == -1 {
            print!(".");
        } else {
            print!("{num}");
        }
    }
    println!()
}
