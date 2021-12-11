use proconio::input;
use proconio::marker::Chars;

fn solve(octopuses: &Vec<Vec<usize>>) -> usize {
    let mut octopuses = octopuses.clone();
    let mut count = 0;

    for _ in 0..100 {
        count += simulate(&mut octopuses);
    }
    count
}

fn solve2(octopuses: &Vec<Vec<usize>>) -> usize {
    let mut octopuses = octopuses.clone();

    (1..)
        .find(|_| simulate(&mut octopuses) == octopuses.len() * octopuses[0].len())
        .unwrap()
}

fn simulate(octopuses: &mut Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    let mut stack = Vec::new();

    for i in 0..octopuses.len() {
        for j in 0..octopuses[0].len() {
            octopuses[i][j] = (octopuses[i][j] + 1) % 10;
            if octopuses[i][j] == 0 {
                stack.push((i, j));
                count += 1;
            }
        }
    }

    while let Some((i, j)) = stack.pop() {
        for (dx, dy) in [
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
        {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || octopuses.len() as i32 <= x
                || y < 0
                || octopuses[0].len() as i32 <= y
                || octopuses[x as usize][y as usize] == 0
            {
                continue;
            }
            octopuses[x as usize][y as usize] = (octopuses[x as usize][y as usize] + 1) % 10;
            if octopuses[x as usize][y as usize] == 0 {
                stack.push((x as usize, y as usize));
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        octopuses: [Chars; 10]
    }

    let octopuses = octopuses
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&octopuses));
    println!("part2: {}", solve2(&octopuses));
}
