use std::collections::{HashSet, VecDeque};

use proconio::input;
use proconio::marker::Chars;

fn check_blizzard(
    i: usize,
    j: usize,
    t: usize,
    left: &Vec<Vec<usize>>,
    right: &Vec<Vec<usize>>,
    up: &Vec<Vec<usize>>,
    down: &Vec<Vec<usize>>,
) -> bool {
    left[i]
        .iter()
        .any(|l| ((up.len() - 2) + (l - 1) - t % (up.len() - 2)) % (up.len() - 2) == j - 1)
        || right[i]
            .iter()
            .any(|r| (r - 1 + t) % (up.len() - 2) == j - 1)
        || up[j].iter().any(|u| {
            ((left.len() - 2) + (u - 1) - t % (left.len() - 2)) % (left.len() - 2) == i - 1
        })
        || down[j]
            .iter()
            .any(|c| (c - 1 + t) % (left.len() - 2) == i - 1)
}

fn bfs(
    start: (usize, usize),
    goal: (usize, usize),
    time: usize,
    left: &Vec<Vec<usize>>,
    right: &Vec<Vec<usize>>,
    up: &Vec<Vec<usize>>,
    down: &Vec<Vec<usize>>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, time));
    let mut visited = HashSet::new();

    while let Some(((i, j), t)) = queue.pop_front() {
        if (i, j) == goal {
            return t;
        }
        if visited.contains(&(i, j, t)) {
            continue;
        }
        visited.insert((i, j, t));
        let move_step = if (i, j) == (0, 1) {
            vec![(0, 0), (1, 0)]
        } else if (i, j) == (left.len() - 1, up.len() - 2) {
            vec![(0, 0), (-1, 0)]
        } else {
            vec![(0, 0), (-1, 0), (0, -1), (0, 1), (1, 0)]
        };
        for (dx, dy) in move_step.iter() {
            let (x, y) = ((i as i32 + dx) as usize, ((j as i32 + dy) as usize));
            if ((x, y) != start
                && (x, y) != goal
                && (!(1..left.len() - 1).contains(&x) || !(1..up.len() - 1).contains(&y)))
                || check_blizzard(x, y, t + 1, left, right, up, down)
            {
                continue;
            }
            queue.push_back(((x, y), t + 1));
        }
    }

    unreachable!()
}

fn solve1(
    left: &Vec<Vec<usize>>,
    right: &Vec<Vec<usize>>,
    up: &Vec<Vec<usize>>,
    down: &Vec<Vec<usize>>,
) -> usize {
    bfs(
        (0, 1),
        (left.len() - 1, up.len() - 2),
        0,
        left,
        right,
        up,
        down,
    )
}

fn solve2(
    left: &Vec<Vec<usize>>,
    right: &Vec<Vec<usize>>,
    up: &Vec<Vec<usize>>,
    down: &Vec<Vec<usize>>,
) -> usize {
    let t1 = bfs(
        (0, 1),
        (left.len() - 1, up.len() - 2),
        0,
        left,
        right,
        up,
        down,
    );
    let t2 = bfs(
        (left.len() - 1, up.len() - 2),
        (0, 1),
        t1,
        left,
        right,
        up,
        down,
    );
    bfs(
        (0, 1),
        (left.len() - 1, up.len() - 2),
        t2,
        left,
        right,
        up,
        down,
    )
}
fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    let mut left = vec![vec![]; map.len()];
    let mut right = vec![vec![]; map.len()];
    let mut up = vec![vec![]; map[0].len()];
    let mut down = vec![vec![]; map[0].len()];

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            match map[i][j] {
                '<' => left[i].push(j),
                '>' => right[i].push(j),
                '^' => up[j].push(i),
                'v' => down[j].push(i),
                _ => {}
            }
        }
    }

    println!("{}", solve1(&left, &right, &up, &down));
    println!("{}", solve2(&left, &right, &up, &down));
}
