use regex::Regex;
use std::collections::BTreeMap;

struct Solution {
    rules: BTreeMap<String, Vec<Vec<i32>>>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

impl Solution {
    fn new(s: String) -> Self {
        let mut splitted = s.split("\n\n");

        let re = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let rules = splitted
            .next()
            .unwrap()
            .split('\n')
            .map(|l| {
                let caps = re.captures(l).unwrap();
                (
                    caps.get(1)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                    vec![
                        vec![
                            caps.get(2)
                                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                            caps.get(3)
                                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                        ],
                        vec![
                            caps.get(4)
                                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                            caps.get(5)
                                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                        ],
                    ],
                )
            })
            .collect::<BTreeMap<String, Vec<Vec<i32>>>>();

        let my_ticket = splitted
            .next()
            .unwrap()
            .strip_prefix("your ticket:\n")
            .unwrap()
            .split(',')
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let nearby_tickets = splitted
            .next()
            .unwrap()
            .strip_prefix("nearby tickets:\n")
            .unwrap()
            .split('\n')
            .map(|l| {
                l.split(',')
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Self {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }

    fn solve(&self) -> i32 {
        self.nearby_tickets
            .iter()
            .map(|n| {
                n.iter()
                    .filter(|t| {
                        !self.rules.values().any(|r| {
                            (&r[0][0] <= *t && *t <= &r[0][1]) || (&r[1][0] <= *t && *t <= &r[1][1])
                        })
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }

    fn solve2(&self) -> i64 {
        let filtered = self
            .nearby_tickets
            .iter()
            .filter(|n| {
                n.iter().all(|t| {
                    self.rules.values().any(|r| {
                        (&r[0][0] <= t && t <= &r[0][1]) || (&r[1][0] <= t && t <= &r[1][1])
                    })
                })
            })
            .collect::<Vec<&Vec<i32>>>();

        let mut counts = vec![vec![0; filtered.len()]; self.rules.len()];

        for n in filtered {
            for i in 0..n.len() {
                for (j, (_, r)) in self.rules.iter().enumerate() {
                    if (r[0][0] <= n[i] && n[i] <= r[0][1]) || (r[1][0] <= n[i] && n[i] <= r[1][1])
                    {
                        counts[j][i] += 1;
                    }
                }
            }
        }

        let l = counts[0].len();
        let mut candidates = counts
            .into_iter()
            .map(|c| (0..c.len()).filter(|i| l == c[*i]).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();

        let mut decided = Vec::new();
        while candidates.iter().any(|c| c.len() > 1) {
            for i in 0..candidates.len() {
                if candidates[i].len() == 1 {
                    if !decided.contains(&candidates[i][0]) {
                        decided.push(candidates[i][0]);
                    }
                } else {
                    candidates[i] = candidates[i]
                        .iter()
                        .filter(|c| !decided.contains(c))
                        .cloned()
                        .collect::<Vec<usize>>();
                }
            }
        }

        self.rules
            .keys()
            .enumerate()
            .filter(|(_, k)| k.starts_with("departure"))
            .map(|(i, _)| self.my_ticket[candidates[i][0]] as i64)
            .product::<i64>()
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
        let str1 = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

        let solution = Solution::new(str1.to_string());
        assert_eq!(71, solution.solve());
    }
}
