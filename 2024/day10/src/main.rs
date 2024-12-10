use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }
    let map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn solve1(map: &Vec<Vec<usize>>) -> usize {
    let mut stack = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                stack.push(((i as i32, j as i32), (i, j)));
            }
        }
    }

    let mut score = HashSet::new();

    while let Some(((i, j), s)) = stack.pop() {
        if map[i as usize][j as usize] == 9 {
            score.insert((s, (i, j)));
            continue;
        }
        for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i + di, j + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize] != map[i as usize][j as usize] + 1
            {
                continue;
            }
            stack.push(((new_i, new_j), s));
        }
    }

    score.len()
}

fn solve2(map: &Vec<Vec<usize>>) -> usize {
    let mut stack = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                stack.push(((i as i32, j as i32), (i, j)));
            }
        }
    }

    let mut score = HashMap::new();

    while let Some(((i, j), s)) = stack.pop() {
        if map[i as usize][j as usize] == 9 {
            *score.entry(s).or_insert(0) += 1;
            continue;
        }
        for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i + di, j + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize] != map[i as usize][j as usize] + 1
            {
                continue;
            }
            stack.push(((new_i, new_j), s));
        }
    }

    score.values().sum::<usize>()
}
