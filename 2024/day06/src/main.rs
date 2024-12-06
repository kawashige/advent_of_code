use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut visited = map.clone();

    let mut pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                pos = (i as i32, j as i32);
            }
        }
    }
    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d_i = 0;

    loop {
        visited[pos.0 as usize][pos.1 as usize] = 'X';
        let mut new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
        if !(0..visited.len() as i32).contains(&new_pos.0)
            || !(0..visited[0].len() as i32).contains(&new_pos.1)
        {
            break;
        }
        while visited[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            d_i = (d_i + 1) % d.len();
            new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
        }
        pos = new_pos;
    }

    let mut count = 0;
    for i in 0..visited.len() {
        for j in 0..visited[0].len() {
            if visited[i][j] == 'X' {
                count += 1;
            }
        }
    }
    count
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut visited = map.clone();

    let mut start_pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                start_pos = (i as i32, j as i32);
            }
        }
    }
    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d_i = 0;
    let mut pos = start_pos;

    loop {
        visited[pos.0 as usize][pos.1 as usize] = 'X';
        let mut new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
        if !(0..visited.len() as i32).contains(&new_pos.0)
            || !(0..visited[0].len() as i32).contains(&new_pos.1)
        {
            break;
        }
        while visited[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            d_i = (d_i + 1) % d.len();
            new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
        }
        pos = new_pos;
    }

    let mut candidates = Vec::new();
    for i in 0..visited.len() {
        for j in 0..visited[0].len() {
            if visited[i][j] == 'X' {
                candidates.push((i, j));
            }
        }
    }

    let mut count = 0;

    for (c_i, c_j) in candidates {
        let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        let mut pos = start_pos;
        let mut d_i = 0;

        loop {
            if !(0..map.len() as i32).contains(&pos.0) || !(0..map[0].len() as i32).contains(&pos.1)
            {
                break;
            }
            if visited[pos.0 as usize][pos.1 as usize][d_i] {
                count += 1;
                break;
            }
            visited[pos.0 as usize][pos.1 as usize][d_i] = true;
            let mut new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
            while ((0..map.len() as i32).contains(&new_pos.0)
                && (0..map[0].len() as i32).contains(&new_pos.1))
                && (map[new_pos.0 as usize][new_pos.1 as usize] == '#'
                    || new_pos == (c_i as i32, c_j as i32))
            {
                d_i = (d_i + 1) % d.len();
                new_pos = (pos.0 as i32 + d[d_i].0, pos.1 as i32 + d[d_i].1);
            }
            pos = new_pos;
        }
    }

    count
}
