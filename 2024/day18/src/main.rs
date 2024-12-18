use std::{collections::VecDeque, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let list = buf
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&list));
    println!("{:?}", solve2(&list));
}

fn solve1(list: &Vec<Vec<usize>>) -> usize {
    let mut map = vec![vec![false; 71]; 71];
    let mut seen = vec![vec![false; 71]; 71];
    for i in 0..1024 {
        map[list[i][0] as usize][list[i][1]] = true;
    }
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    while let Some(((i, j), c)) = queue.pop_front() {
        if (i, j) == (70, 70) {
            return c;
        }
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;

        for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if !(0..map.len() as i32).contains(&new_i)
                || !(0..map[0].len() as i32).contains(&new_j)
                || map[new_i as usize][new_j as usize]
            {
                continue;
            }
            queue.push_back(((new_i as usize, new_j as usize), c + 1));
        }
    }

    unreachable!()
}

fn solve2(list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut map = vec![vec![false; 71]; 71];
    for i in 0..1024 {
        map[list[i][0] as usize][list[i][1]] = true;
    }

    for k in 1024..list.len() {
        let mut seen = vec![vec![false; 71]; 71];
        map[list[k][0] as usize][list[k][1] as usize] = true;
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut can_exit = false;

        while let Some((i, j)) = queue.pop_front() {
            if (i, j) == (70, 70) {
                can_exit = true;
                break;
            }
            if seen[i][j] {
                continue;
            }
            seen[i][j] = true;

            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..map.len() as i32).contains(&new_i)
                    || !(0..map[0].len() as i32).contains(&new_j)
                    || map[new_i as usize][new_j as usize]
                {
                    continue;
                }
                queue.push_back((new_i as usize, new_j as usize));
            }
        }
        if !can_exit {
            return list[k].clone();
        }
    }

    unreachable!()
}
