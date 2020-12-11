struct Solution {}

impl Solution {
    fn solve(init: &Vec<Vec<char>>) -> i32 {
        fn occupied_counts(seats: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
            let moves: [[i32; 2]; 8] = [
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1],
            ];
            let mut counts = vec![vec![0; seats[0].len()]; seats.len()];

            for i in 0..seats.len() {
                for j in 0..seats[0].len() {
                    if seats[i][j] == '#' {
                        for v in moves
                            .iter()
                            .map(|v| [i as i32 + v[0], j as i32 + v[1]])
                            .filter(|v| {
                                0 <= v[0]
                                    && v[0] < seats.len() as i32
                                    && 0 <= v[1]
                                    && v[1] < seats[0].len() as i32
                            })
                        {
                            counts[v[0] as usize][v[1] as usize] += 1;
                        }
                    }
                }
            }
            counts
        }

        let mut seats = init.clone();
        let mut count = 0;
        let mut changed = true;
        while changed {
            changed = false;
            count = 0;
            let counts = occupied_counts(&seats);
            for i in 0..seats.len() {
                for j in 0..seats[0].len() {
                    if seats[i][j] == 'L' {
                        if counts[i][j] == 0 {
                            seats[i][j] = '#';
                            count += 1;
                            changed = true;
                        }
                    } else if seats[i][j] == '#' {
                        if 3 < counts[i][j] {
                            seats[i][j] = 'L';
                            changed = true;
                        } else {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn solve2(init: &Vec<Vec<char>>) -> i32 {
        let mut seats = init.clone();
        let moves: [[i32; 2]; 8] = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];
        let mut directions = Vec::new();
        for i in 0..seats.len() {
            directions.push(Vec::new());
            for j in 0..seats[0].len() {
                let mut positions = Vec::new();
                if seats[i][j] != '.' {
                    for v in &moves {
                        let mut p = [i as i32, j as i32];
                        while 0 <= p[0] + v[0]
                            && p[0] + v[0] < seats.len() as i32
                            && 0 <= p[1] + v[1]
                            && p[1] + v[1] < seats[0].len() as i32
                            && ((p[0] == i as i32 && p[1] == j as i32)
                                || seats[p[0] as usize][p[1] as usize] == '.')
                        {
                            p[0] += v[0];
                            p[1] += v[1];
                        }
                        if !(p[0] == i as i32 && p[1] == j as i32)
                            && seats[p[0] as usize][p[1] as usize] != '.'
                        {
                            positions.push(p);
                        }
                    }
                }
                directions.last_mut().unwrap().push(positions);
            }
        }

        let mut count = 0;
        let mut changed = true;
        while changed {
            let current = seats.clone();
            changed = false;
            count = 0;
            for i in 0..seats.len() {
                for j in 0..seats[0].len() {
                    let c = directions[i][j]
                        .iter()
                        .filter(|v| current[v[0] as usize][v[1] as usize] == '#')
                        .count();
                    if seats[i][j] == 'L' {
                        if c == 0 {
                            seats[i][j] = '#';
                            count += 1;
                            changed = true;
                        }
                    } else if seats[i][j] == '#' {
                        if 4 < c {
                            seats[i][j] = 'L';
                            changed = true;
                        } else {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let seats = buf
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>();
    println!("Part1: {}", Solution::solve(&seats));
    println!("Part2: {}", Solution::solve2(&seats));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str1 = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

        let input1 = str1
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<_>>>();

        assert_eq!(37, Solution::solve(&input1));
        assert_eq!(26, Solution::solve2(&input1));
    }
}
