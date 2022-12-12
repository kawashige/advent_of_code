use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn elevation(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    ((match grid[i][j] {
        'S' => 'a',
        'E' => 'z',
        _ => grid[i][j],
    }) as u8
        - b'a') as i32
}

fn solve1(grid: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                queue.push_back((0, (i, j)));
            }
        }
    }
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

    while let Some((step, (i, j))) = queue.pop_front() {
        if grid[i][j] == 'E' {
            return step;
        }
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || y < 0
                || grid.len() as i32 <= x
                || grid[0].len() as i32 <= y
                || seen[x as usize][y as usize]
                || 1 < elevation(grid, x as usize, y as usize) - elevation(grid, i, j)
            {
                continue;
            }
            queue.push_back((step + 1, (x as usize, y as usize)));
        }
    }

    unreachable!()
}

fn solve2(grid: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'E' {
                queue.push_back((0, (i, j)));
            }
        }
    }
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

    while let Some((step, (i, j))) = queue.pop_front() {
        if grid[i][j] == 'a' || grid[i][j] == 'S' {
            return step;
        }
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || y < 0
                || grid.len() as i32 <= x
                || grid[0].len() as i32 <= y
                || seen[x as usize][y as usize]
                || 1 < elevation(grid, i, j) - elevation(grid, x as usize, y as usize)
            {
                continue;
            }
            queue.push_back((step + 1, (x as usize, y as usize)));
        }
    }

    unreachable!()
}
fn main() {
    input! {
        n: usize,
        grid: [Chars; n]
    }

    println!("{}", solve1(&grid));
    println!("{}", solve2(&grid));
}
