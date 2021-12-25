use std::collections::HashMap;

use proconio::input;

#[derive(Debug)]
enum Operation {
    Inp(String),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Mod(String, String),
    Eql(String, String),
}

fn solve(operations: &Vec<Operation>) -> i64 {
    calc(operations, true)
}

fn solve2(operations: &Vec<Operation>) -> i64 {
    calc(operations, false)
}

fn calc(operations: &Vec<Operation>, max: bool) -> i64 {
    let mut map: HashMap<(i64, i64, i64), i64> = vec![((0, 0, 0), 0)].into_iter().collect();
    let inp_indices = (0..operations.len())
        .filter(|i| match operations[*i] {
            Operation::Inp(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    for i in 0..inp_indices.len() {
        let mut new_map = HashMap::new();
        let end_index = if i == inp_indices.len() - 1 {
            operations.len()
        } else {
            inp_indices[i + 1]
        };
        for w in 1..10 {
            for ((x, y, z), model_number) in &map {
                let mut values = [w, *x, *y, *z];
                apply(&operations[(inp_indices[i] + 1)..end_index], &mut values);
                let entry = new_map
                    .entry((values[1], values[2], values[3]))
                    .or_insert(if max { 0 } else { std::i64::MAX });
                *entry = if max {
                    std::cmp::max(*entry, model_number * 10 + w)
                } else {
                    std::cmp::min(*entry, model_number * 10 + w)
                };
            }
        }
        map = new_map;
    }

    map.into_iter()
        .filter(|((_, _, z), _)| z == &0)
        .map(|(_, v)| v)
        .max()
        .unwrap()
}

fn apply(operations: &[Operation], values: &mut [i64; 4]) {
    for operation in operations {
        match &operation {
            Operation::Add(v1, v2) => {
                values[variable_index(&v1)] += get_value(&v2, &values);
            }
            Operation::Mul(v1, v2) => {
                values[variable_index(&v1)] *= get_value(&v2, &values);
            }
            Operation::Div(v1, v2) => {
                values[variable_index(&v1)] /= get_value(&v2, &values);
            }
            Operation::Mod(v1, v2) => {
                values[variable_index(&v1)] %= get_value(&v2, &values);
            }
            Operation::Eql(v1, v2) => {
                values[variable_index(&v1)] =
                    if values[variable_index(&v1)] == get_value(&v2, &values) {
                        1
                    } else {
                        0
                    };
            }
            _ => unreachable!(),
        }
    }
}

fn get_value(x: &str, values: &[i64; 4]) -> i64 {
    if is_variable(x) {
        values[variable_index(x)]
    } else {
        x.parse::<i64>().unwrap()
    }
}

fn is_variable(x: &str) -> bool {
    x.chars().next().unwrap().is_ascii_alphabetic()
}

fn variable_index(x: &str) -> usize {
    x.chars().next().unwrap() as usize - b'w' as usize
}

fn main() {
    input! {
        n: usize,
    }

    let mut operations = Vec::new();
    for _ in 0..n {
        input! {
            op: String
        }
        if op == "inp" {
            input! {
                v1: String
            }
            operations.push(Operation::Inp(v1))
        } else {
            input! {
                v1: String,
                v2: String
            }
            operations.push(match op.as_str() {
                "add" => Operation::Add(v1, v2),
                "mul" => Operation::Mul(v1, v2),
                "div" => Operation::Div(v1, v2),
                "mod" => Operation::Mod(v1, v2),
                "eql" => Operation::Eql(v1, v2),
                _ => unreachable!(),
            });
        }
    }

    // println!("{:?}", operations);

    println!("part1: {}", solve(&operations));
    println!("part2: {}", solve2(&operations));
}
