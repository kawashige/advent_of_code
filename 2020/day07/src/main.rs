struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    fn solve(map: &HashMap<String, Vec<String>>) -> usize {
        let mut opened = HashSet::new();
        let mut current = vec!["shiny gold".to_string()];
        while !current.is_empty() {
            let mut next = Vec::new();
            for c in current {
                if opened.contains(&c) {
                    continue;
                }
                opened.insert(c.clone());
                if let Some(bags) = map.get(&c) {
                    for b in bags {
                        next.push(b.to_string());
                    }
                }
            }
            current = next;
        }

        opened.len() - 1
    }

    fn solve2(map: &HashMap<String, Vec<(i32, String)>>) -> usize {
        let mut current = vec![(1, "shiny gold".to_string())];
        let mut result = 0;
        while !current.is_empty() {
            let mut next = Vec::new();
            for c in current {
                if let Some(bags) = map.get(&c.1) {
                    for b in bags {
                        let count = b.0 * c.0;
                        result += count;
                        next.push((count, b.1.to_string()));
                    }
                }
            }
            current = next;
        }

        result as usize
    }

    fn str_to_input(input: Vec<&str>) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        for s in input {
            if s.ends_with("no other bags.") {
                continue;
            }
            let mut splitted = s.split(" bags contain ");
            let key = splitted.next().unwrap().to_string();
            for b in splitted.next().unwrap().split(", ").map(|b| {
                let mut sp = b
                    .trim_end_matches('.')
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag")
                    .split(" ");
                (
                    sp.next().unwrap().parse::<i32>().unwrap(),
                    sp.collect::<Vec<&str>>().join(" "),
                )
            }) {
                (*map.entry(b.1).or_insert(Vec::new())).push(key.clone());
            }
        }
        map
    }

    fn str_to_input2(input: Vec<&str>) -> HashMap<String, Vec<(i32, String)>> {
        let mut map = HashMap::new();
        for s in input {
            if s.ends_with("no other bags.") {
                continue;
            }
            let mut splitted = s.split(" bags contain ");
            let key = splitted.next().unwrap().to_string();
            let contained = splitted
                .next()
                .unwrap()
                .split(", ")
                .map(|b| {
                    let mut sp = b
                        .trim_end_matches('.')
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag")
                        .split(" ");
                    (
                        sp.next().unwrap().parse::<i32>().unwrap(),
                        sp.collect::<Vec<&str>>().join(" "),
                    )
                })
                .collect::<Vec<(i32, String)>>();

            map.insert(key, contained);
        }
        map
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let str = buf.split("\n").collect::<Vec<&str>>();
    let input = Solution::str_to_input(str.clone());
    let input2 = Solution::str_to_input2(str);
    println!("Part1: {}", Solution::solve(&input));
    println!("Part2: {}", Solution::solve2(&input2));
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day07() {
        let s = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let input = Solution::str_to_input(s.split("\n").collect::<Vec<&str>>());
        assert_eq!(4, Solution::solve(&input));

        let input = Solution::str_to_input2(s.split("\n").collect::<Vec<&str>>());
        assert_eq!(32, Solution::solve2(&input));

        let s = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

        let input = Solution::str_to_input2(s.split("\n").collect::<Vec<&str>>());
        assert_eq!(126, Solution::solve2(&input));
    }
}
