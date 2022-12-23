use std::collections::{HashMap, HashSet};

use proconio::input;
use proconio::marker::Chars;

fn count_empty_field(elves: &HashSet<(i32, i32)>) -> i32 {
    let ((row_min, row_max), (column_min, column_max)) = elves.iter().fold(
        (
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
        ),
        |((row_min, row_max), (column_min, column_max)), (i, j)| {
            (
                (row_min.min(*i), row_max.max(*i)),
                (column_min.min(*j), column_max.max(*j)),
            )
        },
    );
    (row_max - row_min + 1) * (column_max - column_min + 1) - elves.len() as i32
}

fn propose_direction(pos: (i32, i32), round: usize, elves: &HashSet<(i32, i32)>) -> (i32, i32) {
    if [
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
    .all(|(i, j)| !elves.contains(&(pos.0 + i, pos.1 + j)))
    {
        return pos;
    }

    let directions = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
    ];
    let step = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for d in 0..directions.len() {
        if directions[(d + round) % directions.len()]
            .iter()
            .all(|(i, j)| !elves.contains(&(pos.0 + i, pos.1 + j)))
        {
            let move_step = step[(d + round) % step.len()];
            return (pos.0 + move_step.0, pos.1 + move_step.1);
        }
    }

    pos
}

fn solve1(elves: &HashSet<(i32, i32)>) -> i32 {
    let mut elves = elves.clone();

    for round in 0..10 {
        let mut propose_pos_count = HashMap::new();
        let mut next_elves = Vec::new();
        for pos in &elves {
            let proposed = propose_direction(pos.clone(), round, &elves);
            *propose_pos_count.entry(proposed).or_insert(0) += 1;
            next_elves.push((pos.clone(), proposed));
        }
        elves = next_elves
            .into_iter()
            .map(|(pos, proposed)| {
                if propose_pos_count[&proposed] == 1 {
                    proposed
                } else {
                    pos
                }
            })
            .collect::<HashSet<_>>();
    }

    count_empty_field(&elves)
}

fn solve2(elves: &HashSet<(i32, i32)>) -> usize {
    let mut elves = elves.clone();

    for round in 0.. {
        let mut propose_pos_count = HashMap::new();
        let mut next_elves = Vec::new();
        let mut not_moved = 0;
        for pos in &elves {
            let proposed = propose_direction(pos.clone(), round, &elves);
            *propose_pos_count.entry(proposed).or_insert(0) += 1;
            if pos == &proposed {
                not_moved += 1;
            }
            next_elves.push((pos.clone(), proposed));
        }
        if not_moved == elves.len() {
            return round + 1;
        }
        elves = next_elves
            .into_iter()
            .map(|(pos, proposed)| {
                if propose_pos_count[&proposed] == 1 {
                    proposed
                } else {
                    pos
                }
            })
            .collect::<HashSet<_>>();
    }

    unreachable!()
}

fn main() {
    input! {
        n: usize,
        grove: [Chars; n]
    }

    let mut elves = HashSet::new();
    for i in 0..grove.len() {
        for j in 0..grove[0].len() {
            if grove[i][j] == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    println!("{}", solve1(&elves));
    println!("{}", solve2(&elves));
}
