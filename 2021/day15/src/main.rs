use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Chars;

fn solve(risk_level: &Vec<Vec<usize>>) -> usize {
    lower_risk(risk_level)
}

fn solve2(risk_level: &Vec<Vec<usize>>) -> usize {
    let mut actual_risk_level = vec![vec![0; risk_level[0].len() * 5]; risk_level.len() * 5];
    for i in 0..actual_risk_level.len() {
        for j in 0..actual_risk_level[0].len() {
            actual_risk_level[i][j] = (risk_level[i % risk_level.len()][j % risk_level[0].len()]
                + i / risk_level.len()
                + j / risk_level[0].len())
                % 9;
            if actual_risk_level[i][j] == 0 {
                actual_risk_level[i][j] = 9;
            }
        }
    }

    lower_risk(&actual_risk_level)
}

fn lower_risk(risk_level: &Vec<Vec<usize>>) -> usize {
    let mut dist = vec![vec![std::usize::MAX; risk_level[0].len()]; risk_level.len()];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0)));

    while let Some((Reverse(risk), (i, j))) = heap.pop() {
        if dist[i][j] < risk {
            continue;
        }

        if (i, j) == (risk_level.len() - 1, risk_level[0].len() - 1) {
            return risk;
        }

        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (r, c) = (i as i32 + dx, j as i32 + dy);
            if r < 0
                || risk_level.len() as i32 <= r
                || c < 0
                || risk_level[0].len() as i32 <= c
                || dist[r as usize][c as usize] <= risk + risk_level[r as usize][c as usize]
            {
                continue;
            }
            dist[r as usize][c as usize] = risk + risk_level[r as usize][c as usize];
            heap.push((
                Reverse(dist[r as usize][c as usize]),
                (r as usize, c as usize),
            ));
        }
    }

    unreachable!();
}

fn main() {
    input! {
        n: usize,
        risk_level: [Chars; n]
    }

    let risk_level = risk_level
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&risk_level));
    println!("part2: {}", solve2(&risk_level));
}
