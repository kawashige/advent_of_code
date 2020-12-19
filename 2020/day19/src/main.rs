#[derive(Debug, Clone)]
enum Rule {
    Character(char),
    SubRules(Vec<Vec<i32>>),
}

use std::collections::HashMap;
struct Solution {
    rules: HashMap<i32, Rule>,
    messages: Vec<String>,
}

use regex::Regex;
impl Solution {
    fn new(s: String) -> Self {
        let mut splitted = s.split("\n\n");

        let re = Regex::new(r"(.+): (.+)").unwrap();
        let rules = splitted
            .next()
            .unwrap()
            .split('\n')
            .map(|l| {
                let caps = re.captures(l).unwrap();
                let key = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let mut value = caps.get(2).unwrap().as_str().to_string();
                if value.starts_with("\"") {
                    (key, Rule::Character(value.remove(1)))
                } else {
                    (
                        key,
                        Rule::SubRules(
                            value
                                .split(" | ")
                                .map(|sub| {
                                    sub.split(' ')
                                        .map(|s| s.parse::<i32>().unwrap())
                                        .collect::<Vec<i32>>()
                                })
                                .collect::<Vec<Vec<i32>>>(),
                        ),
                    )
                }
            })
            .collect::<HashMap<i32, Rule>>();

        let messages = splitted
            .next()
            .unwrap()
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        Self { rules, messages }
    }

    fn recurse(key: i32, map: &HashMap<i32, Rule>, current: Vec<String>) -> Vec<String> {
        match &map[&key] {
            Rule::Character(c) => {
                if current.is_empty() {
                    vec![c.to_string()]
                } else {
                    current
                        .into_iter()
                        .map(|mut s| {
                            s.push(*c);
                            s
                        })
                        .collect::<Vec<String>>()
                }
            }
            Rule::SubRules(v) => v
                .iter()
                .map(|vv| {
                    let mut tmp = current.clone();
                    for s in vv {
                        tmp = Self::recurse(*s, map, tmp);
                    }
                    tmp
                })
                .flatten()
                .collect::<Vec<String>>(),
        }
    }

    fn solve(&self) -> i32 {
        let strings = Self::recurse(0, &self.rules, Vec::new());
        self.messages.iter().filter(|m| strings.contains(m)).count() as i32
    }

    fn solve2(&self) -> i32 {
        let strings_42 = Self::recurse(42, &self.rules, Vec::new());
        let strings_31 = Self::recurse(31, &self.rules, Vec::new());
        let l = strings_31[0].len();
        self.messages
            .iter()
            .filter(|m| {
                let mut i = 0;
                let mut count_31 = 0;
                let mut count_42 = 0;
                while i <= m.len() - l {
                    if strings_42.contains(&m[i..(i + l)].to_string()) {
                        count_42 += 1;
                        i += l;
                    } else {
                        break;
                    }
                }
                let mut j = m.len();
                while l <= j {
                    if strings_31.contains(&m[(j - l)..j].to_string()) {
                        count_31 += 1;
                        j -= l;
                    } else {
                        break;
                    }
                }
                let n = m.len() / l;
                if n < count_42 + count_31 {
                    count_31 -= count_42 + count_31 - n;
                }
                n == count_42 + count_31 && 0 < count_31 && 1 < count_42 && count_31 < count_42
            })
            .count() as i32
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
        let str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let solution = Solution::new(str.to_string());
        assert_eq!(2, solution.solve());

        let str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let solution = Solution::new(str.to_string());
        assert_eq!(12, solution.solve2());
    }
}
