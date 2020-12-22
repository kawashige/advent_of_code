use std::collections::HashSet;
use std::collections::VecDeque;
struct Solution {
    decks: Vec<VecDeque<i32>>,
}

impl Solution {
    fn new(s: String) -> Self {
        let decks = s
            .split("\n\n")
            .map(|p| {
                let mut splitted = p.split('\n');
                splitted.next();
                splitted
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<VecDeque<i32>>()
            })
            .collect::<Vec<VecDeque<i32>>>();
        Self { decks }
    }

    fn calc_score(target: &VecDeque<i32>) -> i32 {
        target.iter().rev().zip(1..).map(|(i, j)| i * j).sum()
    }

    fn solve(&self) -> i32 {
        let mut p1 = self.decks[0].clone();
        let mut p2 = self.decks[1].clone();
        while !p1.is_empty() && !p1.is_empty() {
            let c1 = p1.pop_front().unwrap();
            let c2 = p2.pop_front().unwrap();
            if c1 < c2 {
                p2.push_back(c2);
                p2.push_back(c1);
            } else {
                p1.push_back(c1);
                p1.push_back(c2);
            }
        }
        if p1.is_empty() {
            Self::calc_score(&p2)
        } else {
            Self::calc_score(&p1)
        }
    }

    fn recurse_combat(p1: VecDeque<i32>, p2: VecDeque<i32>) -> (VecDeque<i32>, VecDeque<i32>) {
        let mut p1 = p1;
        let mut p2 = p2;
        let mut state1 = HashSet::new();
        let mut state2 = HashSet::new();

        while !p1.is_empty() && !p2.is_empty() {
            let key1 = p1
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let key2 = p2
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
            if state1.contains(&key1) || state2.contains(&key2) {
                return (p1, VecDeque::new());
            }

            let c1 = p1.pop_front().unwrap();
            let c2 = p2.pop_front().unwrap();
            if c1 <= p1.len() as i32 && c2 <= p2.len() as i32 {
                let sub_game = Self::recurse_combat(
                    p1.iter().take(c1 as usize).cloned().collect(),
                    p2.iter().take(c2 as usize).cloned().collect(),
                );
                if sub_game.0.is_empty() {
                    p2.push_back(c2);
                    p2.push_back(c1);
                } else {
                    p1.push_back(c1);
                    p1.push_back(c2);
                }
            } else if c1 < c2 {
                p2.push_back(c2);
                p2.push_back(c1);
            } else {
                p1.push_back(c1);
                p1.push_back(c2);
            }
            state1.insert(key1);
            state2.insert(key2);
        }
        (p1, p2)
    }

    fn solve2(&self) -> i32 {
        let result = Self::recurse_combat(self.decks[0].clone(), self.decks[1].clone());
        if result.0.is_empty() {
            Self::calc_score(&result.1)
        } else {
            Self::calc_score(&result.0)
        }
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
        let str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let solution = Solution::new(str.to_string());
        assert_eq!(306, solution.solve());
        assert_eq!(291, solution.solve2());

        let str = r#"Player 1:
43
19

Player 2:
2
29
14"#;
        let solution = Solution::new(str.to_string());
        assert_eq!(105, solution.solve2());
    }
}
