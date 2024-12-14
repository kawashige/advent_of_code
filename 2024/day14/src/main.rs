use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let list = buf
        .split("\n")
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|sp| {
                    sp.split(&['=', ','])
                        .filter_map(|d| d.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&list));
    println!("{:?}", solve2(&list));
}

fn solve1(list: &Vec<Vec<Vec<i32>>>) -> usize {
    let mut tile = vec![vec![0; 101]; 103];

    for l in list {
        let mut x = l[0][0];
        let mut y = l[0][1];
        for _ in 0..100 {
            x = ((x + l[1][0]) % tile[0].len() as i32 + tile[0].len() as i32)
                % tile[0].len() as i32;
            y = ((y + l[1][1]) % tile.len() as i32 + tile.len() as i32) % tile.len() as i32;
        }
        tile[y as usize][x as usize] += 1;
    }

    let mut c = 0;
    for i in 0..tile.len() / 2 {
        for j in 0..tile[0].len() / 2 {
            c += tile[i][j];
        }
    }
    let mut result = c;
    c = 0;
    for i in 0..tile.len() / 2 {
        for j in tile[0].len() / 2 + 1..tile[0].len() {
            c += tile[i][j];
        }
    }
    result *= c;
    c = 0;
    for i in tile.len() / 2 + 1..tile.len() {
        for j in 0..tile[0].len() / 2 {
            c += tile[i][j];
        }
    }
    result *= c;
    c = 0;
    for i in tile.len() / 2 + 1..tile.len() {
        for j in tile[0].len() / 2 + 1..tile[0].len() {
            c += tile[i][j];
        }
    }
    result * c
}

fn solve2(list: &Vec<Vec<Vec<i32>>>) -> usize {
    let mut positions = list.iter().map(|v| v[0].clone()).collect::<Vec<_>>();
    let tile = vec![vec![0; 101]; 103];

    for i in 0.. {
        for j in 0..list.len() {
            positions[j][0] = ((positions[j][0] + list[j][1][0]) % tile[0].len() as i32
                + tile[0].len() as i32)
                % tile[0].len() as i32;
            positions[j][1] = ((positions[j][1] + list[j][1][1]) % tile.len() as i32
                + tile.len() as i32)
                % tile.len() as i32;
        }

        let mut tile = vec![vec![0; 101]; 103];
        let mut count = 0;
        for p in &positions {
            tile[p[1] as usize][p[0] as usize] += 1;
            let mut found = false;
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (ni, nj) = (p[1] + di, p[0] + dj);
                if !(0..tile.len() as i32).contains(&ni) || !(0..tile[0].len() as i32).contains(&nj)
                {
                    continue;
                }
                if 0 < tile[ni as usize][nj as usize] {
                    found = true;
                }
            }
            if found {
                count += 1;
            }
        }

        if 250 < count {
            println!("loop: {}, {}", i, count);
            for i in 0..tile.len() {
                println!(
                    "{}",
                    tile[i]
                        .iter()
                        .map(|c| if c == &0 { '.' } else { 'X' })
                        .collect::<String>()
                );
            }
            return i + 1;
        }
    }
    unreachable!()
}
