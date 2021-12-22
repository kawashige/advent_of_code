use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn solve(operations: &Vec<(bool, Vec<(i32, i32)>)>) -> usize {
    let mut cube = vec![vec![vec![false; 101]; 101]; 101];

    for (set, op) in operations {
        for x in op[0].0.max(-50)..=op[0].1.min(50) {
            for y in op[1].0.max(-50)..=op[1].1.min(50) {
                for z in op[2].0.max(-50)..=op[2].1.min(50) {
                    cube[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = *set;
                }
            }
        }
    }

    cube.iter()
        .map(|y| {
            y.iter()
                .map(|z| z.iter().filter(|x| **x).count())
                .sum::<usize>()
        })
        .sum()
}

fn solve2(operations: &Vec<(bool, Vec<(i32, i32)>)>) -> usize {
    let mut x = BTreeSet::new();
    let mut y = BTreeSet::new();
    let mut z = BTreeSet::new();
    for (_, op) in operations {
        x.insert(op[0].0);
        x.insert(op[0].1);
        x.insert(op[0].1 + 1);
        y.insert(op[1].0);
        y.insert(op[1].1);
        y.insert(op[1].1 + 1);
        z.insert(op[2].0);
        z.insert(op[2].1);
        z.insert(op[2].1 + 1);
    }
    let x = x.into_iter().collect::<Vec<_>>();
    let y = y.into_iter().collect::<Vec<_>>();
    let z = z.into_iter().collect::<Vec<_>>();
    let x_map = x
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<HashMap<_, _>>();
    let y_map = y
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<HashMap<_, _>>();
    let z_map = z
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<HashMap<_, _>>();

    let mut cube = vec![vec![vec![false; z.len()]; y.len()]; x.len()];

    for (set, op) in operations {
        for x in x_map[&op[0].0]..=x_map[&op[0].1] {
            for y in y_map[&op[1].0]..=y_map[&op[1].1] {
                for z in z_map[&op[2].0]..=z_map[&op[2].1] {
                    cube[x][y][z] = *set;
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..x.len() {
        for j in 0..y.len() {
            for k in 0..z.len() {
                if cube[i][j][k] {
                    sum += (x[i + 1] - x[i]) as usize
                        * (y[j + 1] - y[j]) as usize
                        * (z[k + 1] - z[k]) as usize;
                }
            }
        }
    }
    sum
}

fn main() {
    input! {
        n: usize,
        operations: [(String, String); n]
    }

    let operations = operations
        .into_iter()
        .map(|(set, ranges)| {
            let set = set == "on";
            let ranges = ranges
                .split(",")
                .map(|r| {
                    let r = r
                        .trim_start_matches("x=")
                        .trim_start_matches("y=")
                        .trim_start_matches("z=")
                        .split("..")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    (r[0], r[1])
                })
                .collect::<Vec<_>>();
            (set, ranges)
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&operations));
    println!("part2: {}", solve2(&operations));
}
