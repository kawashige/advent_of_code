use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    unreachable!()
}

fn next_move(c: char) -> Vec<(i32, i32)> {
    match c {
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(0, -1), (1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        _ => vec![],
    }
}
fn next_move2(i: usize, j: usize, map: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();

    if map[i][j] == 'S' {
        if i == 0 || !['-', 'J', '7'].contains(&map[i - 1][j]) {
            moves.push((-1, 0));
        }
        if j == 0 || !['-', 'F', 'L'].contains(&map[i][j - 1]) {
            moves.push((1, 0));
        }
        if j == 0 || !['|', 'J', 'L'].contains(&map[i][j - 1]) {
            moves.push((0, -1));
        }
        if i == 0 || !['|', 'F', '7'].contains(&map[i - 1][j]) {
            moves.push((0, 1));
        }
    } else {
        if i == 0 || !['-', 'J', '7'].contains(&map[i - 1][j]) {
            moves.push((-1, 0));
        }
        if !['-', 'J', '7'].contains(&map[i][j]) {
            moves.push((1, 0));
        }
        if j == 0 || !['|', 'J', 'L'].contains(&map[i][j - 1]) {
            moves.push((0, -1));
        }
        if !['|', 'J', 'L'].contains(&map[i][j]) {
            moves.push((0, 1));
        }
    }

    moves
}

fn solve1(map: &[Vec<char>]) -> (usize, Vec<Vec<bool>>) {
    let start = find_start(&map);
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    seen[start.0][start.1] = true;
    let mut queue = VecDeque::new();

    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (r, c) = (start.0 as i32 + dr, start.1 as i32 + dc);
        if !(0..map.len() as i32).contains(&r) || !(0..map[0].len() as i32).contains(&c) {
            continue;
        }
        if next_move(map[r as usize][c as usize])
            .contains(&(start.0 as i32 - r, start.1 as i32 - c))
        {
            queue.push_back(((r as usize, c as usize), 1));
        }
    }
    let mut max = 0;

    while let Some(((i, j), step)) = queue.pop_front() {
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        max = max.max(step);

        for (dr, dc) in next_move(map[i][j]) {
            let (r, c) = (i as i32 + dr, j as i32 + dc);
            if !(0..map.len() as i32).contains(&r)
                || !(0..map[0].len() as i32).contains(&c)
                || seen[r as usize][c as usize]
            {
                continue;
            }
            queue.push_back(((r as usize, c as usize), step + 1));
        }
    }

    (max, seen)
}

fn solve2(map: &Vec<Vec<char>>, is_main_loop: &[Vec<bool>]) -> usize {
    let mut map = map.clone();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !is_main_loop[i][j] {
                map[i][j] = '.';
            }
        }
    }
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' && !seen[i][j] {
                let mut count = 0;
                let mut outside = false;
                let mut stack = vec![(i, j)];
                while let Some((r, c)) = stack.pop() {
                    if seen[r][c] {
                        continue;
                    }
                    seen[r][c] = true;
                    if map[r][c] == '.' {
                        count += 1;
                    }
                    for (dr, dc) in next_move2(r, c, &map) {
                        let (new_r, new_c) = (r as i32 + dr, c as i32 + dc);
                        if !(0..map.len() as i32).contains(&new_r)
                            || !(0..map[0].len() as i32).contains(&new_c)
                        {
                            outside = true;
                            continue;
                        }

                        if seen[new_r as usize][new_c as usize] {
                            continue;
                        }
                        stack.push((new_r as usize, new_c as usize));
                    }
                }
                if !outside {
                    result += count;
                }
            }
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    let (count, is_main_loop) = solve1(&map);
    println!("{}", count);
    println!("{}", solve2(&map, &is_main_loop));
}
