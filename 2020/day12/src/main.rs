struct Solution {}

impl Solution {
    fn solve(instructions: &Vec<(char, i32)>) -> i32 {
        let directions = [[1, 0], [0, -1], [-1, 0], [0, 1]];
        let mut pos = [0, 0];
        let mut d = 0;

        for ins in instructions {
            match ins.0 {
                'N' => pos[1] += ins.1,
                'S' => pos[1] -= ins.1,
                'E' => pos[0] += ins.1,
                'W' => pos[0] -= ins.1,
                'L' => {
                    d = (d + directions.len() - (ins.1 as usize / 90) % directions.len())
                        % directions.len()
                }
                'R' => d = (d + ins.1 as usize / 90) % directions.len(),
                'F' => {
                    pos[0] += directions[d][0] * ins.1;
                    pos[1] += directions[d][1] * ins.1;
                }
                _ => unreachable!(),
            }
        }

        pos[0].abs() + pos[1].abs()
    }

    fn solve2(instructions: &Vec<(char, i32)>) -> i32 {
        let mut ship = [0, 0];
        let mut waypoints = [10, 1];

        for ins in instructions {
            match ins.0 {
                'N' => waypoints[1] += ins.1,
                'S' => waypoints[1] -= ins.1,
                'E' => waypoints[0] += ins.1,
                'W' => waypoints[0] -= ins.1,
                'R' | 'L' => {
                    let r = if ins.0 == 'R' {
                        (ins.1 as usize / 90) % 4
                    } else {
                        4 - (ins.1 as usize / 90) % 4
                    };
                    if r == 1 {
                        waypoints = [waypoints[1], -waypoints[0]];
                    } else if r == 2 {
                        waypoints = [-waypoints[0], -waypoints[1]];
                    } else if r == 3 {
                        waypoints = [-waypoints[1], waypoints[0]];
                    }
                }
                'F' => {
                    ship[0] += waypoints[0] * ins.1;
                    ship[1] += waypoints[1] * ins.1;
                }
                _ => unreachable!(),
            }
        }

        ship[0].abs() + ship[1].abs()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let instructions = buf
        .split('\n')
        .map(|s| {
            let mut c = s.chars();
            (
                c.next().unwrap(),
                c.collect::<String>().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(char, i32)>>();
    println!("Part1: {}", Solution::solve(&instructions));
    println!("Part2: {}", Solution::solve2(&instructions));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str1 = r#"F10
N3
F7
R90
F11"#;

        let input1 = str1
            .split('\n')
            .map(|s| {
                let mut c = s.chars();
                (
                    c.next().unwrap(),
                    c.collect::<String>().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<(char, i32)>>();

        assert_eq!(25, Solution::solve(&input1));
        assert_eq!(286, Solution::solve2(&input1));
    }
}
