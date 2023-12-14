use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn tilt_north(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    for j in 0..map[0].len() {
        let mut pos = 0;
        for i in 0..map.len() {
            if map[i][j] == 'O' {
                new_map[pos][j] = 'O';
                pos += 1;
            } else if map[i][j] == '#' {
                pos = i + 1;
                new_map[i][j] = '#';
            }
        }
    }
    new_map
}
fn tilt_south(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    for j in 0..map[0].len() {
        let mut pos = map.len() - 1;
        for i in (0..map.len()).rev() {
            if map[i][j] == 'O' {
                new_map[pos][j] = 'O';
                pos = pos.saturating_sub(1);
            } else if map[i][j] == '#' {
                pos = i.saturating_sub(1);
                new_map[i][j] = '#';
            }
        }
    }
    new_map
}
fn tilt_west(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    for i in 0..map.len() {
        let mut pos = 0;
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                new_map[i][pos] = 'O';
                pos += 1;
            } else if map[i][j] == '#' {
                pos = j + 1;
                new_map[i][j] = '#';
            }
        }
    }
    new_map
}
fn tilt_east(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    for i in 0..map.len() {
        let mut pos = map[0].len() - 1;
        for j in (0..map[0].len()).rev() {
            if map[i][j] == 'O' {
                new_map[i][pos] = 'O';
                pos = pos.saturating_sub(1)
            } else if map[i][j] == '#' {
                pos = j.saturating_sub(1);
                new_map[i][j] = '#';
            }
        }
    }
    new_map
}

fn map_to_key(map: &Vec<Vec<char>>) -> String {
    map.iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<String>()
}

fn calculate_load(map: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                load += map.len() - i;
            }
        }
    }
    load
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut map = map.clone();
    map = tilt_north(&map);
    calculate_load(&map)
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut memo = HashMap::new();
    let mut map = map.clone();
    memo.insert(map_to_key(&map), 0);
    for i in 1.. {
        map = tilt_north(&map);
        map = tilt_west(&map);
        map = tilt_south(&map);
        map = tilt_east(&map);
        let key = map_to_key(&map);
        if let Some(j) = memo.get(&key) {
            let remains = (1_000_000_000 - i) % (i - j);
            for _ in 0..remains {
                map = tilt_north(&map);
                map = tilt_west(&map);
                map = tilt_south(&map);
                map = tilt_east(&map);
            }
            return calculate_load(&map);
        }
        memo.insert(map_to_key(&map), i);
    }
    unreachable!()
}
fn main() {
    input! {
        n: usize,
        map: [Chars; n],
    }

    println!("{}", solve1(&map));
    println!("{}", solve2(&map));
}
