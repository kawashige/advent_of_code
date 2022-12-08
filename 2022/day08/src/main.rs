use proconio::input;
use proconio::marker::Chars;

fn solve1(grid: &Vec<Vec<i32>>) -> usize {
    let mut visible = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        let mut max = -1;
        for j in 0..grid[0].len() {
            if max < grid[i][j] {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for i in 0..grid.len() {
        let mut max = -1;
        for j in (0..grid[0].len()).rev() {
            if max < grid[i][j] {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for j in 0..grid[0].len() {
        let mut max = -1;
        for i in 0..grid.len() {
            if max < grid[i][j] {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for j in 0..grid[0].len() {
        let mut max = -1;
        for i in (0..grid.len()).rev() {
            if max < grid[i][j] {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    visible.into_iter().flatten().filter(|v| *v).count()
}

fn solve2(grid: &Vec<Vec<i32>>) -> usize {
    let mut max_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let up = i
                - (0..i)
                    .rev()
                    .find(|k| grid[i][j] <= grid[*k][j])
                    .unwrap_or(0);
            let left = j
                - (0..j)
                    .rev()
                    .find(|k| grid[i][j] <= grid[i][*k])
                    .unwrap_or(0);
            let right = (j + 1..grid[0].len())
                .find(|k| grid[i][j] <= grid[i][*k])
                .unwrap_or(grid[0].len() - 1)
                - j;
            let down = (i + 1..grid.len())
                .find(|k| grid[i][j] <= grid[*k][j])
                .unwrap_or(grid.len() - 1)
                - i;
            max_score = max_score.max(up * left * right * down);
        }
    }
    max_score
}

fn main() {
    input! {
        n: usize,
        grid: [Chars; n]
    }

    let grid = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|d| d.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&grid));
    println!("{}", solve2(&grid));
}
