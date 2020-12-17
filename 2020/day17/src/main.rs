struct Solution {
    state: Vec<Vec<usize>>,
}

impl Solution {
    fn new(s: String) -> Self {
        let state = s
            .split('\n')
            .map(|l| {
                l.chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Self { state }
    }

    fn solve(&self) -> i32 {
        let neighbors: [[i32; 3]; 26] = [
            [1, -1, -1],
            [1, -1, 0],
            [1, -1, 1],
            [1, 0, -1],
            [1, 0, 0],
            [1, 0, 1],
            [1, 1, -1],
            [1, 1, 0],
            [1, 1, 1],
            [0, -1, -1],
            [0, -1, 0],
            [0, -1, 1],
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, -1],
            [0, 1, 0],
            [0, 1, 1],
            [-1, -1, -1],
            [-1, -1, 0],
            [-1, -1, 1],
            [-1, 0, -1],
            [-1, 0, 0],
            [-1, 0, 1],
            [-1, 1, -1],
            [-1, 1, 0],
            [-1, 1, 1],
        ];

        let cycle = 6;
        let size_y = self.state.len();
        let size_x = self.state[0].len();
        let mut state = vec![vec![vec![0; size_x + 2 * cycle]; size_y + 2 * cycle]; 1 + 2 * cycle];
        for i in 0..size_y {
            for j in 0..size_x {
                if self.state[i][j] == 1 {
                    state[cycle][i + cycle][j + cycle] = 1;
                }
            }
        }

        for _ in 0..cycle {
            let mut next_state =
                vec![vec![vec![0; size_x + 2 * cycle]; size_y + 2 * cycle]; 1 + 2 * cycle];
            for i in 0..state.len() {
                for j in 0..state[0].len() {
                    for k in 0..state[0][0].len() {
                        if state[i][j][k] == 1 {
                            for n in neighbors.iter() {
                                next_state[(i as i32 + n[0]) as usize]
                                    [(j as i32 + n[1]) as usize]
                                    [(k as i32 + n[2]) as usize] += 1;
                            }
                        }
                    }
                }
            }
            for i in 0..state.len() {
                for j in 0..state[0].len() {
                    for k in 0..state[0][0].len() {
                        if state[i][j][k] == 1 {
                            if next_state[i][j][k] == 2 || next_state[i][j][k] == 3 {
                                next_state[i][j][k] = 1;
                            } else {
                                next_state[i][j][k] = 0;
                            }
                        } else {
                            if next_state[i][j][k] == 3 {
                                next_state[i][j][k] = 1;
                            } else {
                                next_state[i][j][k] = 0;
                            }
                        }
                    }
                }
            }
            state = next_state
        }

        state
            .iter()
            .map(|xy| xy.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>())
            .sum::<i32>()
    }

    fn solve2(&self) -> i32 {
        let moves = [0, 1, -1];
        let mut neighbors = Vec::new();
        for w in 0..3 {
            for z in 0..3 {
                for y in 0..3 {
                    for x in 0..3 {
                        if !(x == 0 && y == 0 && z == 0 && w == 0) {
                            neighbors.push([moves[w], moves[z], moves[y], moves[x]]);
                        }
                    }
                }
            }
        }

        let cycle = 6;
        let size_y = self.state.len();
        let size_x = self.state[0].len();
        let mut state =
            vec![
                vec![vec![vec![0; size_x + 2 * cycle]; size_y + 2 * cycle]; 1 + 2 * cycle];
                1 + 2 * cycle
            ];
        for i in 0..size_y {
            for j in 0..size_x {
                if self.state[i][j] == 1 {
                    state[cycle][cycle][i + cycle][j + cycle] = 1;
                }
            }
        }

        for _ in 0..cycle {
            let mut next_state =
                vec![
                    vec![vec![vec![0; size_x + 2 * cycle]; size_y + 2 * cycle]; 1 + 2 * cycle];
                    1 + 2 * cycle
                ];
            for i in 0..state.len() {
                for j in 0..state[0].len() {
                    for k in 0..state[0][0].len() {
                        for l in 0..state[0][0][0].len() {
                            if state[i][j][k][l] == 1 {
                                for n in neighbors.iter() {
                                    next_state[(i as i32 + n[0]) as usize]
                                        [(j as i32 + n[1]) as usize]
                                        [(k as i32 + n[2]) as usize]
                                        [(l as i32 + n[3]) as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            for i in 0..state.len() {
                for j in 0..state[0].len() {
                    for k in 0..state[0][0].len() {
                        for l in 0..state[0][0][0].len() {
                            if state[i][j][k][l] == 1 {
                                if next_state[i][j][k][l] == 2 || next_state[i][j][k][l] == 3 {
                                    next_state[i][j][k][l] = 1;
                                } else {
                                    next_state[i][j][k][l] = 0;
                                }
                            } else {
                                if next_state[i][j][k][l] == 3 {
                                    next_state[i][j][k][l] = 1;
                                } else {
                                    next_state[i][j][k][l] = 0;
                                }
                            }
                        }
                    }
                }
            }
            state = next_state
        }

        state
            .iter()
            .map(|zxy| {
                zxy.iter()
                    .map(|xy| xy.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>())
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let solution = Solution::new(buf);
    println!("Part1: {}", solution.solve());
    println!("Part2: {}", solution.solve2());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str1 = r#".#.
..#
###2"#;

        let solution = Solution::new(str1.to_string());
        assert_eq!(112, solution.solve());
        assert_eq!(848, solution.solve2());
    }
}
