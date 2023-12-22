use std::{collections::HashSet, io::Read};

fn parse_input(s: String) -> Vec<Vec<Vec<i32>>> {
    s.split('\n')
        .map(|line| {
            line.split('~')
                .map(|pos| {
                    pos.split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_blocked(snapshot: &Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut snapshot = snapshot.clone();
    snapshot.sort_unstable_by_key(|s| s[0][2].min(s[1][2]));

    let max = snapshot.iter().fold([0, 0, 0], |mut max, brick| {
        for i in 0..3 {
            max[i] = max[i].max(brick[0][i]).max(brick[1][i]);
        }
        max
    });

    let mut pos =
        vec![vec![vec![-1; max[2] as usize + 1]; max[1] as usize + 1]; max[0] as usize + 1];
    let mut blocked = vec![vec![]; snapshot.len()];

    for i in 0..snapshot.len() {
        if snapshot[i][0][0] != snapshot[i][1][0] {
            // x
            let mut z = snapshot[i][0][2] as usize;
            let x_min = snapshot[i][0][0].min(snapshot[i][1][0]);
            let x_max = snapshot[i][0][0].max(snapshot[i][1][0]);
            while 1 < z {
                let bricks = (x_min..=x_max)
                    .map(|x| pos[x as usize][snapshot[i][0][1] as usize][z - 1])
                    .filter(|j| j != &-1)
                    .collect::<HashSet<_>>();
                if !bricks.is_empty() {
                    blocked[i] = bricks.into_iter().collect();
                    break;
                }
                z -= 1;
            }
            for x in x_min..=x_max {
                pos[x as usize][snapshot[i][0][1] as usize][z] = i as i32;
            }
        } else if snapshot[i][0][1] != snapshot[i][1][1] {
            // y
            let mut z = snapshot[i][0][2] as usize;
            let y_min = snapshot[i][0][1].min(snapshot[i][1][1]);
            let y_max = snapshot[i][0][1].max(snapshot[i][1][1]);
            while 1 < z {
                let bricks = (y_min..=y_max)
                    .map(|y| pos[snapshot[i][0][0] as usize][y as usize][z - 1])
                    .filter(|j| j != &-1)
                    .collect::<HashSet<_>>();
                if !bricks.is_empty() {
                    blocked[i] = bricks.into_iter().collect();
                    break;
                }
                z -= 1;
            }
            for y in y_min..=y_max {
                pos[snapshot[i][0][0] as usize][y as usize][z] = i as i32;
            }
        } else {
            // z
            let mut z = snapshot[i][0][2].min(snapshot[i][1][2]) as usize;
            while 1 < z {
                if pos[snapshot[i][0][0] as usize][snapshot[i][0][1] as usize][z - 1] != -1 {
                    blocked[i] =
                        vec![pos[snapshot[i][0][0] as usize][snapshot[i][0][1] as usize][z - 1]];
                    break;
                }
                z -= 1;
            }
            for j in 0..(snapshot[i][0][2] - snapshot[i][1][2]).abs() + 1 {
                pos[snapshot[i][0][0] as usize][snapshot[i][0][1] as usize][z + j as usize] =
                    i as i32;
            }
        }
    }

    blocked
}

fn solve1(blocked: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;
    for i in 0..blocked.len() {
        if blocked
            .iter()
            .filter(|bricks| bricks.len() == 1 && bricks[0] == i as i32)
            .count()
            == 0
        {
            count += 1;
        }
    }

    count
}

fn solve2(blocked: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    for i in 0..blocked.len() {
        let mut removed = vec![false; blocked.len()];
        removed[i] = true;
        let mut stack = vec![i];
        while let Some(j) = stack.pop() {
            removed[j] = true;
            for k in 0..blocked.len() {
                if !removed[k]
                    && !blocked[k].is_empty()
                    && blocked[k].iter().all(|b| removed[*b as usize])
                {
                    removed[k] = true;
                    stack.push(k);
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let snapshot = parse_input(buf);
    let blocked = get_blocked(&snapshot);
    println!("{}", solve1(&blocked));
    println!("{}", solve2(&blocked));
}
