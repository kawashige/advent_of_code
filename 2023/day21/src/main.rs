use std::collections::{HashSet, VecDeque};

use proconio::input;
use proconio::marker::Chars;

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut start = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    const STEP: usize = 64;

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut seen = vec![vec![vec![false; STEP + 1]; map[0].len()]; map.len()];
    let mut count = 0;

    while let Some(((i, j), step)) = queue.pop_front() {
        if seen[i][j][step] {
            continue;
        }
        seen[i][j][step] = true;
        if step == STEP {
            count += 1;
            continue;
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || seen[new_i as usize][new_j as usize][step + 1]
                || map[new_i as usize][new_j as usize] == '#'
            {
                continue;
            }
            queue.push_back(((new_i as usize, new_j as usize), step + 1));
        }
    }

    count
}

fn count(map: &Vec<Vec<char>>, max_step: usize, start: (i32, i32)) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut count = 0;
    let size = map.len() as i32;

    while let Some(((i, j), step)) = queue.pop_front() {
        if max_step < step {
            continue;
        }
        if seen.contains(&(i, j)) {
            continue;
        }
        seen.insert((i, j));
        if step % 2 == max_step % 2 {
            count += 1;
        }
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (new_i, new_j) = (i + di, j + dj);
            if map[(((new_i % size) + size) % size) as usize]
                [(((new_j % size) + size) % size) as usize]
                == '#'
            {
                continue;
            }
            queue.push_back(((new_i, new_j), step + 1));
        }
    }

    count
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut start = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }
    const STEP: usize = 26501365;
    let len = STEP % map.len();
    let c1 = count(map, len, start);
    let c2 = count(map, len + map.len(), start);
    let c3 = count(map, len + map.len() * 2, start);

    println!("c1: {}", c1);
    println!("c2: {}", c2);
    println!("c3: {}", c3);

    let x = STEP / map.len();
    let y = (c1 - 2 * c2 + c3) / 2;
    let z = (4 * c2 - 3 * c1 - c3) / 2;

    y * x * x + z * x + c1
}

fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    println!("{}", solve1(&map));
    println!("{}", solve2(&map));
}
