use std::collections::HashSet;

use proconio::input;

fn is_adjacent(p1: (i32, i32), p2: (i32, i32)) -> bool {
    (p1.0 - p2.0).abs() < 2 && (p1.1 - p2.1).abs() < 2
}

fn pull(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    if is_adjacent(h, t) {
        return t;
    }
    (
        t.0 + if t.0 < h.0 {
            1
        } else if h.0 < t.0 {
            -1
        } else {
            0
        },
        t.1 + if t.1 < h.1 {
            1
        } else if h.1 < t.1 {
            -1
        } else {
            0
        },
    )
}

fn solve1(motions: &[(char, usize)]) -> usize {
    simulate(motions, 2)
}

fn solve2(motions: &[(char, usize)]) -> usize {
    simulate(motions, 10)
}

fn simulate(motions: &[(char, usize)], l: usize) -> usize {
    let mut rope = vec![(0, 0); l];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (direction, distance) in motions {
        for _ in 0..*distance {
            match direction {
                'U' => rope[0] = (rope[0].0 + 1, rope[0].1),
                'D' => rope[0] = (rope[0].0 - 1, rope[0].1),
                'L' => rope[0] = (rope[0].0, rope[0].1 - 1),
                'R' => rope[0] = (rope[0].0, rope[0].1 + 1),
                _ => unreachable!(),
            }

            for i in 1..rope.len() {
                rope[i] = pull(rope[i - 1], rope[i]);
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }

    visited.len()
}

fn main() {
    input! {
        n: usize,
        motions: [(char, usize); n]
    }

    println!("{}", solve1(&motions));
    println!("{}", solve2(&motions));
}
