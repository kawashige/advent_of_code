use std::{cmp::Reverse, collections::BinaryHeap, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let map = buf
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let score = solve1(&map);
    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map, score));
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

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s, 0));

    let mut min_cost = vec![vec![vec![std::usize::MAX; 4]; map[0].len()]; map.len()];

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((Reverse(score), (i, j), d)) = heap.pop() {
        if map[i][j] == 'E' {
            return score;
        }
        if min_cost[i][j][d] <= score {
            continue;
        }
        min_cost[i][j][d] = score;

        for k in 0..directions.len() {
            let (new_i, new_j) = (i as i32 + directions[k].0, j as i32 + directions[k].1);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize] == '#'
            {
                continue;
            }

            heap.push((
                Reverse(
                    score
                        + 1
                        + 1000
                            * (d as i32 - k as i32)
                                .abs()
                                .min(4 - (d as i32 - k as i32).abs())
                                as usize,
                ),
                (new_i as usize, new_j as usize),
                k,
            ));
        }
    }

    unreachable!()
}

fn recurse(
    i: usize,
    j: usize,
    d: usize,
    score: usize,
    lowest_score: usize,
    map: &Vec<Vec<char>>,
    min_cost: &mut Vec<Vec<Vec<usize>>>,
    paths: &mut Vec<Vec<bool>>,
) -> bool {
    if map[i][j] == 'E' && score == lowest_score {
        paths[i][j] = true;
        return true;
    }
    if min_cost[i][j][d] < score {
        return false;
    }
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    min_cost[i][j][d] = score;

    let mut result = false;
    for k in 0..directions.len() {
        let (new_i, new_j) = (i as i32 + directions[k].0, j as i32 + directions[k].1);
        if !(0..map.len() as i32).contains(&new_i)
            || !(0..map[0].len() as i32).contains(&new_j)
            || map[new_i as usize][new_j as usize] == '#'
        {
            continue;
        }

        let new_score = score
            + 1
            + 1000
                * (d as i32 - k as i32)
                    .abs()
                    .min(4 - (d as i32 - k as i32).abs()) as usize;
        if recurse(
            new_i as usize,
            new_j as usize,
            k,
            new_score,
            lowest_score,
            map,
            min_cost,
            paths,
        ) {
            paths[i][j] = true;
            result |= true;
        }
    }

    result
}

fn solve2(map: &Vec<Vec<char>>, lowest_score: usize) -> usize {
    let mut s = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                s = (i, j);
            }
        }
    }

    let mut min_cost = vec![vec![vec![std::usize::MAX; 4]; map[0].len()]; map.len()];
    let mut paths = vec![vec![false; map[0].len()]; map.len()];
    paths[s.0][s.1] = true;

    recurse(s.0, s.1, 0, 0, lowest_score, map, &mut min_cost, &mut paths);

    let mut result = 0;
    for i in 0..paths.len() {
        for j in 0..paths[0].len() {
            if paths[i][j] {
                result += 1;
            }
        }
    }
    result
}
