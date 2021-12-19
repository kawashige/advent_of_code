use std::{collections::HashSet, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(scanners: &Vec<Vec<(i32, i32, i32)>>) -> usize {
    let (fixed, _) = calc(scanners);

    fixed.iter().flatten().collect::<HashSet<_>>().len()
}

fn solve2(scanners: &Vec<Vec<(i32, i32, i32)>>) -> i32 {
    let (_, coodinates) = calc(scanners);

    (0..coodinates.len())
        .map(|i| {
            ((i + 1)..coodinates.len())
                .map(|j| manhattan_distance(coodinates[i], coodinates[j]))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
}

fn manhattan_distance(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

fn calc(scanners: &Vec<Vec<(i32, i32, i32)>>) -> (Vec<Vec<(i32, i32, i32)>>, Vec<(i32, i32, i32)>) {
    let mut fixed = vec![scanners[0].clone()];
    let mut unfixed = (1..scanners.len()).collect::<Vec<_>>();
    let mut coodinates = vec![(0, 0, 0)];

    while !unfixed.is_empty() {
        let mut new_unfixed = Vec::new();
        for i in unfixed {
            let mut is_fixed = false;
            'outer: for j in 0..24 {
                let scanner = scanners[i]
                    .iter()
                    .map(|scanner| rotate(scanner, j))
                    .collect::<Vec<_>>();

                for m in 0..fixed.len() {
                    for k in 0..fixed[m].len() {
                        for l in 0..scanner.len() {
                            let d = (
                                fixed[m][k].0 - scanner[l].0,
                                fixed[m][k].1 - scanner[l].1,
                                fixed[m][k].2 - scanner[l].2,
                            );

                            let moved_scanner = scanner
                                .iter()
                                .map(|(x, y, z)| (x + d.0, y + d.1, z + d.2))
                                .collect::<Vec<_>>();

                            let overlap = moved_scanner
                                .iter()
                                .filter(|p| fixed[m].contains(*p))
                                .count();

                            if overlap >= 12 {
                                println!("{}: {:?}", i, d);
                                coodinates.push(d);
                                is_fixed = true;
                                fixed.push(moved_scanner);
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !is_fixed {
                new_unfixed.push(i);
            }
        }
        unfixed = new_unfixed;
    }

    (fixed, coodinates)
}

fn rotate(v: &(i32, i32, i32), pattern: usize) -> (i32, i32, i32) {
    match pattern {
        0 => (v.0, v.2, -v.1),
        1 => (v.1, v.2, v.0),
        2 => (-v.0, -v.1, v.2),
        3 => (-v.1, -v.0, -v.2),
        4 => (-v.0, v.1, -v.2),
        5 => (v.2, -v.0, -v.1),
        6 => (-v.2, v.0, -v.1),
        7 => (-v.2, -v.0, v.1),
        8 => (v.1, -v.0, v.2),
        9 => (-v.2, -v.1, -v.0),
        10 => (v.0, v.1, v.2),
        11 => (v.0, -v.1, -v.2),
        12 => (v.2, v.1, -v.0),
        13 => (v.2, -v.1, v.0),
        14 => (-v.0, -v.2, -v.1),
        15 => (-v.0, v.2, v.1),
        16 => (-v.1, v.2, -v.0),
        17 => (-v.1, v.0, v.2),
        18 => (v.1, v.0, -v.2),
        19 => (v.2, v.0, v.1),
        20 => (-v.1, -v.2, v.0),
        21 => (v.1, -v.2, -v.0),
        22 => (-v.2, v.1, v.0),
        23 => (v.0, -v.2, v.1),
        _ => unreachable!(),
    }
}

fn main() {
    let mut scanners = Vec::new();
    let n: usize = read();
    for _ in 0..n {
        let line: String = read();
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            scanners.push(Vec::new());
        } else {
            let v = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (*scanners.last_mut().unwrap()).push((v[0], v[1], v[2]));
        }
    }

    println!("part1: {}", solve(&scanners));
    println!("part2: {}", solve2(&scanners));
}
