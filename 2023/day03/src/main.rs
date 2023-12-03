use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::Chars;

fn is_adjacent_to_symbol(schematic: &[Vec<char>], i: usize, j: usize) -> bool {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        let (r, c) = (i as i32 + dr, j as i32 + dc);
        if !(0..schematic.len() as i32).contains(&r)
            || !(0..schematic[0].len() as i32).contains(&c)
            || schematic[r as usize][c as usize].is_ascii_digit()
            || schematic[r as usize][c as usize] == '.'
        {
            continue;
        }
        return true;
    }
    false
}

fn get_adjacent_gears(schematic: &[Vec<char>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut gears = Vec::new();
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        let (r, c) = (i as i32 + dr, j as i32 + dc);
        if !(0..schematic.len() as i32).contains(&r)
            || !(0..schematic[0].len() as i32).contains(&c)
            || schematic[r as usize][c as usize] != '*'
        {
            continue;
        }
        gears.push((r as usize, c as usize))
    }
    gears
}

fn solve1(schematic: &[Vec<char>]) -> usize {
    let mut sum = 0;

    for i in 0..schematic.len() {
        let mut num = 0;
        let mut is_adjacent = false;
        for j in 0..schematic[0].len() {
            if schematic[i][j].is_ascii_digit() {
                num = num * 10 + schematic[i][j].to_digit(10).unwrap() as usize;
                is_adjacent |= is_adjacent_to_symbol(&schematic, i, j);
            }
            if !schematic[i][j].is_ascii_digit() || j == schematic[0].len() - 1 {
                if is_adjacent {
                    sum += num;
                }
                num = 0;
                is_adjacent = false;
            }
        }
    }

    sum
}

fn solve2(schematic: &[Vec<char>]) -> usize {
    let mut gears = HashMap::new();

    for i in 0..schematic.len() {
        let mut num = 0;
        let mut adjacent_gears = HashSet::new();
        for j in 0..schematic[0].len() {
            if schematic[i][j].is_ascii_digit() {
                num = num * 10 + schematic[i][j].to_digit(10).unwrap() as usize;
                for (r, c) in get_adjacent_gears(&schematic, i, j) {
                    adjacent_gears.insert((r, c));
                }
            }
            if !schematic[i][j].is_ascii_digit() || j == schematic[0].len() - 1 {
                for (r, c) in adjacent_gears {
                    (*gears.entry((r, c)).or_insert(Vec::new())).push(num);
                }
                num = 0;
                adjacent_gears = HashSet::new();
            }
        }
    }

    gears
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum()
}
fn main() {
    input! {
        n: usize,
        schematic: [Chars; n]
    }

    println!("{}", solve1(&schematic));
    println!("{}", solve2(&schematic));
}
