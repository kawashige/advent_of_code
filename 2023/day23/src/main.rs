use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::Chars;

fn next_move(c: char, is_slippy: bool) -> Vec<(i32, i32)> {
    if is_slippy {
        match c {
            '^' => vec![(-1, 0)],
            'v' => vec![(1, 0)],
            '<' => vec![(0, -1)],
            '>' => vec![(0, 1)],
            _ => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        }
    } else {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
    }
}

fn recurse(
    i: usize,
    j: usize,
    map: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    is_slippy: bool,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    if i == map.len() - 1 {
        return 0;
    }

    let mut step = -1000000;

    for (di, dj) in next_move(map[i][j], is_slippy) {
        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
        if !(0..map.len() as i32).contains(&new_i)
            || !(0..map[0].len() as i32).contains(&new_j)
            || seen[new_i as usize][new_j as usize]
            || map[new_i as usize][new_j as usize] == '#'
        {
            continue;
        }
        seen[new_i as usize][new_j as usize] = true;
        step = step.max(recurse(new_i as usize, new_j as usize, map, seen, is_slippy, memo) + 1);
        seen[new_i as usize][new_j as usize] = false;
    }
    // memo.insert(key, step);

    step
}

fn solve1(map: &Vec<Vec<char>>) -> i32 {
    let start = (
        0,
        (0..map[0].len()).find(|j| map[0][*j] == '.').unwrap() as usize,
    );
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    seen[start.0][start.1] = true;

    recurse(start.0, start.1, map, &mut seen, true, &mut HashMap::new())
}

fn find_next_pos(
    i: usize,
    j: usize,
    branch: &Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    step: i32,
) -> Vec<((usize, usize), i32)> {
    if branch.contains(&(i, j)) {
        return vec![((i, j), step)];
    }

    let mut result = Vec::new();

    for (di, dj) in next_move(map[i][j], false) {
        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
        if !(0..map.len() as i32).contains(&new_i)
            || !(0..map[0].len() as i32).contains(&new_j)
            || seen[new_i as usize][new_j as usize]
            || map[new_i as usize][new_j as usize] == '#'
        {
            continue;
        }
        seen[new_i as usize][new_j as usize] = true;
        result.append(&mut find_next_pos(
            new_i as usize,
            new_j as usize,
            branch,
            map,
            seen,
            step + 1,
        ));
        seen[new_i as usize][new_j as usize] = false;
    }
    result
}

fn recurse2(
    pos: (usize, usize),
    next_pos: &HashMap<(usize, usize), Vec<((usize, usize), i32)>>,
    goal: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> i32 {
    if pos == goal {
        return 0;
    }

    let mut step = -10000000;

    for (next, s) in &next_pos[&pos] {
        if seen.contains(next) {
            continue;
        }
        seen.insert(*next);
        step = step.max(recurse2(*next, next_pos, goal, seen) + s);
        seen.remove(&next);
    }

    step
}

fn solve2(map: &Vec<Vec<char>>) -> i32 {
    let start = (0..map[0].len()).find(|j| map[0][*j] == '.').unwrap() as usize;
    let end = (0..map[0].len())
        .find(|j| map[map.len() - 1][*j] == '.')
        .unwrap() as usize;
    let mut branch = Vec::new();
    branch.push((0, start));
    branch.push((map.len() - 1, end));

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                continue;
            }
            let mut count = 0;

            for (di, dj) in next_move(map[i][j], false) {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if (0..map.len() as i32).contains(&new_i)
                    && (0..map[0].len() as i32).contains(&new_j)
                    && map[new_i as usize][new_j as usize] != '#'
                {
                    count += 1;
                }
            }
            if 2 < count {
                branch.push((i, j));
            }
        }
    }
    let mut next_pos = HashMap::new();
    for i in 0..branch.len() {
        let mut seen = vec![vec![false; map[0].len()]; map.len()];
        seen[branch[i].0][branch[i].1] = true;
        next_pos.insert(
            branch[i],
            find_next_pos(
                branch[i].0,
                branch[i].1,
                &(0..branch.len())
                    .filter_map(|j| if j == i { None } else { Some(branch[j]) })
                    .collect(),
                map,
                &mut seen,
                0,
            ),
        );
    }

    recurse2(
        (0, start),
        &next_pos,
        (map.len() - 1, end),
        &mut HashSet::new(),
    )
}
fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    println!("{}", solve1(&map));
    println!("{}", solve2(&map));
}
