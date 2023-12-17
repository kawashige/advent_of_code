use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Chars;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next_move(direction: Direction) -> Vec<(Direction, (i32, i32))> {
    match direction {
        Direction::Up | Direction::Down => {
            vec![(Direction::Left, (0, -1)), (Direction::Right, (0, 1))]
        }
        Direction::Left | Direction::Right => {
            vec![(Direction::Up, (-1, 0)), (Direction::Down, (1, 0))]
        }
    }
}

fn solve1(map: &Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0), Direction::Right));
    heap.push((Reverse(0), (0, 0), Direction::Down));
    let mut min_losses = vec![vec![vec![std::usize::MAX; 4]; map[0].len()]; map.len()];

    while let Some((Reverse(loss), (i, j), direction)) = heap.pop() {
        if min_losses[i][j][direction as usize] <= loss {
            continue;
        }
        min_losses[i][j][direction as usize] = loss;
        if i == map.len() - 1 && j == map[0].len() - 1 {
            continue;
        }
        for (next_direction, (di, dj)) in get_next_move(direction) {
            let mut next_loss = loss;
            for d in 1..=3 {
                let (next_i, next_j) = (i as i32 + di * d, j as i32 + dj * d);
                if !(0..map.len() as i32).contains(&next_i)
                    || !(0..map[0].len() as i32).contains(&next_j)
                {
                    continue;
                }
                next_loss += map[next_i as usize][next_j as usize];
                heap.push((
                    Reverse(next_loss),
                    (next_i as usize, next_j as usize),
                    next_direction,
                ));
            }
        }
    }

    *min_losses[map.len() - 1][map[0].len() - 1]
        .iter()
        .min()
        .unwrap()
}

fn solve2(map: &Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0), Direction::Right));
    heap.push((Reverse(0), (0, 0), Direction::Down));
    let mut min_losses = vec![vec![vec![std::usize::MAX; 4]; map[0].len()]; map.len()];

    while let Some((Reverse(loss), (i, j), direction)) = heap.pop() {
        if min_losses[i][j][direction as usize] <= loss {
            continue;
        }
        min_losses[i][j][direction as usize] = loss;
        if i == map.len() - 1 && j == map[0].len() - 1 {
            continue;
        }
        for (next_direction, (di, dj)) in get_next_move(direction) {
            let mut next_loss = loss;
            for d in 1..=10 {
                let (next_i, next_j) = (i as i32 + di * d, j as i32 + dj * d);
                if !(0..map.len() as i32).contains(&next_i)
                    || !(0..map[0].len() as i32).contains(&next_j)
                {
                    break;
                }
                next_loss += map[next_i as usize][next_j as usize];
                if d <= 3 {
                    continue;
                }
                heap.push((
                    Reverse(next_loss),
                    (next_i as usize, next_j as usize),
                    next_direction,
                ));
            }
        }
    }

    *min_losses[map.len() - 1][map[0].len() - 1]
        .iter()
        .min()
        .unwrap()
}
fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }
    let map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&map));
    println!("{}", solve2(&map));
}
