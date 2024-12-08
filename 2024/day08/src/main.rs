use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        map: [Chars; n],

    }

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut antennas = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                continue;
            }
            (*antennas.entry(map[i][j]).or_insert(Vec::new())).push((i, j));
        }
    }

    let mut antinodes = HashSet::new();

    for (_, positions) in antennas {
        for j in 0..positions.len() {
            for k in 0..j {
                let antinode1 = (
                    positions[j].0 as i32 + (positions[j].0 as i32 - positions[k].0 as i32),
                    positions[j].1 as i32 + (positions[j].1 as i32 - positions[k].1 as i32),
                );
                if (0..map.len() as i32).contains(&antinode1.0)
                    && (0..map[0].len() as i32).contains(&antinode1.1)
                {
                    antinodes.insert(antinode1);
                }

                let antinode2 = (
                    positions[k].0 as i32 + (positions[k].0 as i32 - positions[j].0 as i32),
                    positions[k].1 as i32 + (positions[k].1 as i32 - positions[j].1 as i32),
                );
                if (0..map.len() as i32).contains(&antinode2.0)
                    && (0..map[0].len() as i32).contains(&antinode2.1)
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes.len()
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut antinodes = HashSet::new();
    let mut antennas = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                continue;
            }
            antinodes.insert((i as i32, j as i32));
            (*antennas.entry(map[i][j]).or_insert(Vec::new())).push((i, j));
        }
    }

    for (_, positions) in antennas {
        for j in 0..positions.len() {
            for k in 0..j {
                let mut antinode1 = (positions[j].0 as i32, positions[j].1 as i32);
                let d = (
                    (positions[j].0 as i32 - positions[k].0 as i32),
                    (positions[j].1 as i32 - positions[k].1 as i32),
                );
                while (0..map.len() as i32).contains(&antinode1.0)
                    && (0..map[0].len() as i32).contains(&antinode1.1)
                {
                    antinode1 = (antinode1.0 as i32 + d.0, antinode1.1 as i32 + d.1);
                    if (0..map.len() as i32).contains(&antinode1.0)
                        && (0..map[0].len() as i32).contains(&antinode1.1)
                    {
                        antinodes.insert(antinode1);
                    }
                }

                let mut antinode2 = (positions[k].0 as i32, positions[k].1 as i32);
                let d = (
                    (positions[k].0 as i32 - positions[j].0 as i32),
                    (positions[k].1 as i32 - positions[j].1 as i32),
                );
                while (0..map.len() as i32).contains(&antinode2.0)
                    && (0..map[0].len() as i32).contains(&antinode2.1)
                {
                    antinode2 = (antinode2.0 as i32 + d.0, antinode2.1 as i32 + d.1);
                    if (0..map.len() as i32).contains(&antinode2.0)
                        && (0..map[0].len() as i32).contains(&antinode2.1)
                    {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
    }

    antinodes.len()
}
