use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let map = buf
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut s = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                s = (i, j);
            }
        }
    }

    let mut path = vec![vec![std::usize::MAX; map[0].len()]; map[0].len()];
    let mut stack = vec![(s, 0)];
    while let Some(((i, j), t)) = stack.pop() {
        if path[i][j] <= t {
            continue;
        }
        path[i][j] = t;
        for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize] == '#'
            {
                continue;
            }
            stack.push(((new_i as usize, new_j as usize), t + 1));
        }
    }
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '#' {
                continue;
            }
            let mut times = Vec::new();
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..map.len() as i32).contains(&new_i)
                    || !(0..map[0].len() as i32).contains(&new_j)
                    || map[new_i as usize][new_j as usize] == '#'
                {
                    continue;
                }
                times.push(path[new_i as usize][new_j as usize]);
            }
            times.sort_unstable();
            for k in 0..times.len() {
                for l in 0..k {
                    if 100 + 2 <= times[k] - times[l] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut s = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                s = (i, j);
            }
        }
    }

    let mut list = Vec::new();
    let mut path = vec![vec![std::usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    queue.push_back((s, 0));
    while let Some(((i, j), t)) = queue.pop_front() {
        if path[i][j] <= t {
            continue;
        }
        path[i][j] = t;
        list.push((i, j));
        for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize] == '#'
            {
                continue;
            }
            queue.push_back(((new_i as usize, new_j as usize), t + 1));
        }
    }

    let mut count = 0;
    const N: usize = 100;
    const T: usize = 20;

    for x in 0..list.len() {
        for y in 0..x {
            if (list[x].0 as i32 - list[y].0 as i32).abs()
                + (list[x].1 as i32 - list[y].1 as i32).abs()
                <= T as i32
            {
                if N + (list[x].0 as i32 - list[y].0 as i32).abs() as usize
                    + (list[x].1 as i32 - list[y].1 as i32).abs() as usize
                    <= x - y
                {
                    count += 1;
                }
            }
        }
    }
    count
}
