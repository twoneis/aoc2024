use std::collections::HashMap;
use std::ops::Shl;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Formula {
    a: String,
    b: String,
    gate: Gate,
    r: String,
}

impl Formula {
    fn new(a: String, b: String, r: String, gate: Gate) -> Formula {
        Formula { a, b, gate, r }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>");
        return;
    }

    let lines: Vec<String> = std::fs::read_to_string(&args[1])
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect();

    let mut assignments: HashMap<String, u8> = HashMap::new();
    let rex = Regex::new(r"(?<var>\w+): (?<val>\w+)").expect("Regex error");
    for line in &lines {
        let Some(caps) = rex.captures(line) else {
            continue;
        };

        let var: String = (&caps["var"]).to_string();
        let val: u8 = (&caps["val"]).parse().expect("Parsing value error");

        assignments.insert(var, val);
    }

    let mut formulas: Vec<Formula> = Vec::new();
    let rex = Regex::new(r"(?<a>\w+) (?<gate>\w+) (?<b>\w+) -> (?<r>\w+)").expect("Regex error");
    for line in &lines {
        let Some(caps) = rex.captures(line) else {
            continue;
        };

        let gate = match &caps["gate"] {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => panic!("Unknown gate"),
        };

        formulas.push(Formula::new(
            (&caps["a"]).to_string(),
            (&caps["b"]).to_string(),
            (&caps["r"]).to_string(),
            gate,
        ));
    }

    for _ in 0..formulas.len() {
        for formula in &formulas {
            let res = assignments.get(&formula.r);
            let a = assignments.get(&formula.a);
            let b = assignments.get(&formula.b);
            match formula.gate {
                Gate::AND => match res {
                    Some(1) => {
                        assignments.insert(formula.a.clone(), 1);
                        assignments.insert(formula.b.clone(), 1);
                    }
                    Some(0) => match a {
                        Some(1) => {
                            assignments.insert(formula.b.clone(), 0);
                        }
                        None => match b {
                            Some(1) => {
                                assignments.insert(formula.a.clone(), 0);
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => match a {
                        Some(0) => {
                            assignments.insert(formula.r.clone(), 0);
                        }
                        Some(1) => match b {
                            Some(1) => {
                                assignments.insert(formula.r.clone(), 1);
                            }
                            Some(0) => {
                                assignments.insert(formula.r.clone(), 0);
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                },
                Gate::OR => match res {
                    Some(0) => {
                        assignments.insert(formula.a.clone(), 0);
                        assignments.insert(formula.b.clone(), 0);
                    }
                    Some(1) => match a {
                        Some(0) => {
                            assignments.insert(formula.b.clone(), 1);
                        }
                        None => match b {
                            Some(0) => {
                                assignments.insert(formula.a.clone(), 1);
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => match a {
                        Some(1) => {
                            assignments.insert(formula.r.clone(), 1);
                        }
                        Some(0) => match b {
                            Some(1) => {
                                assignments.insert(formula.r.clone(), 1);
                            }
                            Some(0) => {
                                assignments.insert(formula.r.clone(), 0);
                            }
                            _ => {}
                        },
                        _ => match b {
                            Some(1) => {
                                assignments.insert(formula.r.clone(), 1);
                            }
                            _ => {}
                        },
                    },
                },
                Gate::XOR => match res {
                    Some(0) => match a {
                        Some(0) => {
                            assignments.insert(formula.b.clone(), 0);
                        }
                        Some(1) => {
                            assignments.insert(formula.b.clone(), 1);
                        }
                        _ => match b {
                            Some(0) => {
                                assignments.insert(formula.a.clone(), 0);
                            }
                            Some(1) => {
                                assignments.insert(formula.a.clone(), 1);
                            }
                            _ => {}
                        },
                    },
                    Some(1) => match a {
                        Some(0) => {
                            assignments.insert(formula.b.clone(), 1);
                        }
                        Some(1) => {
                            assignments.insert(formula.b.clone(), 0);
                        }
                        _ => match b {
                            Some(0) => {
                                assignments.insert(formula.a.clone(), 1);
                            }
                            Some(1) => {
                                assignments.insert(formula.a.clone(), 0);
                            }
                            _ => {}
                        },
                    },
                    _ => match a {
                        Some(1) => match b {
                            Some(1) => {
                                assignments.insert(formula.r.clone(), 0);
                            }
                            Some(0) => {
                                assignments.insert(formula.r.clone(), 1);
                            }
                            _ => {}
                        },
                        Some(0) => match b {
                            Some(1) => {
                                assignments.insert(formula.r.clone(), 1);
                            }
                            Some(0) => {
                                assignments.insert(formula.r.clone(), 0);
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                },
            };
        }
    }

    let z = assignments
        .iter()
        .filter_map(|(k, &v)| {
            let rex = Regex::new(r"z(?<bit>\d+)").expect("Regex error");
            match rex.captures(k) {
                Some(caps) => {
                    let bit: usize = (&caps["bit"]).parse().expect("Parsing error");
                    let shifted: usize = (v as usize).shl(bit);
                    Some(shifted)
                }
                _ => None,
            }
        })
        .fold(0, |mut acc, x| {
            acc += x;
            acc
        });

    let z_values = assignments
        .iter()
        .filter(|(k, _)| {
            let rex = Regex::new(r"z(?<bit>\d+)").expect("Regex error");
            match rex.captures(k) {
                Some(_) => true,
                _ => false,
            }
        })
        .collect::<Vec<(&String, &u8)>>();

    dbg!(z_values);

    println!("Part 1: {z}");
}
