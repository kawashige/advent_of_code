use std::collections::{HashMap, HashSet};

use proconio::input;

fn solve1(plan: &Vec<(char, usize, String)>) -> usize {
    let mut map = vec![vec![false; 1000]; 1000];
    let mut pos = (500, 500);
    map[pos.0 as usize][pos.1 as usize] = true;

    for (direction, len, _) in plan {
        let (di, dj) = match direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        for _ in 0..*len {
            pos.0 += di;
            pos.1 += dj;
            map[pos.0 as usize][pos.1 as usize] = true;
        }
    }
    let mut count = 0;
    let mut stack = vec![(0, 0)];
    let mut seen = vec![vec![false; map[0].len()]; map.len()];

    while let Some((i, j)) = stack.pop() {
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        count += 1;
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || seen[new_i as usize][new_j as usize]
                || map[new_i as usize][new_j as usize]
            {
                continue;
            }
            stack.push((new_i as usize, new_j as usize));
        }
    }

    map.len() * map[0].len() - count
}

fn solve2(plan: &Vec<(char, usize, String)>) -> usize {
    let instructions = plan
        .iter()
        .map(|(_, _, color)| {
            let len = usize::from_str_radix(&color[2..=6], 16).unwrap();
            let direction = match color.as_bytes()[7] {
                b'0' => 'R',
                b'1' => 'D',
                b'2' => 'L',
                b'3' => 'U',
                _ => unreachable!(),
            };
            (direction, len)
        })
        .collect::<Vec<_>>();

    let mut x = HashSet::new();
    let mut y = HashSet::new();
    let mut pos = (0, 0);
    x.insert(pos.0);
    y.insert(pos.1);

    for (direction, len) in &instructions {
        let (di, dj) = match direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        pos = (pos.0 + di * *len as i32, pos.1 + dj * *len as i32);
        x.insert(pos.0);
        x.insert(pos.0 + 1);
        y.insert(pos.1);
        y.insert(pos.1 + 1);
    }
    let mut x_ = x.into_iter().collect::<Vec<_>>();
    let mut y_ = y.into_iter().collect::<Vec<_>>();
    x_.sort_unstable();
    x_.push(x_[0] - 1);
    x_.sort_unstable();
    y_.sort_unstable();
    y_.push(y_[0] - 1);
    y_.sort_unstable();
    let x = x_.clone().into_iter().zip(0..).collect::<HashMap<_, _>>();
    let y = y_.clone().into_iter().zip(0..).collect::<HashMap<_, _>>();

    let mut map = vec![vec![false; y.len()]; x.len()];

    let mut pos = (0, 0);
    map[x[&pos.0]][y[&pos.1]] = true;
    for (direction, len) in instructions {
        let (di, dj) = match direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        let next_pos = (pos.0 + di * len as i32, pos.1 + dj * len as i32);
        for k in 0..(x[&next_pos.0] as i32 - x[&pos.0] as i32)
            .abs()
            .max((y[&next_pos.1] as i32 - y[&pos.1] as i32).abs())
        {
            map[(x[&pos.0] as i32 + di * k) as usize][(y[&pos.1] as i32 + dj * k) as usize] = true;
        }
        pos = next_pos
    }

    let mut stack = vec![(0, 0)];
    let mut seen = vec![vec![false; map[0].len()]; map.len()];

    while let Some((i, j)) = stack.pop() {
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || seen[new_i as usize][new_j as usize]
                || map[new_i as usize][new_j as usize]
            {
                continue;
            }
            stack.push((new_i as usize, new_j as usize));
        }
    }

    let mut count = 0;
    for i in 0..seen.len() {
        for j in 0..seen[0].len() {
            if !seen[i][j] {
                count += (x_[i + 1] - x_[i]) as usize * (y_[j + 1] - y_[j]) as usize;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        plan: [(char, usize, String); n]
    }

    println!("{}", solve1(&plan));
    println!("{}", solve2(&plan));
}
