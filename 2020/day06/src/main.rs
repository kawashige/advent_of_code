struct Solution {}

use std::collections::HashSet;
impl Solution {
    fn solve(input: &Vec<String>) -> usize {
        input
            .iter()
            .map(|s| s.chars().collect::<HashSet<char>>().len())
            .sum()
    }

    fn solve2(input: &Vec<Vec<&str>>) -> usize {
        input
            .iter()
            .map(|v| {
                let mut chars = v[0].chars().collect::<HashSet<char>>();
                for i in 1..v.len() {
                    chars = chars
                        .intersection(&v[i].chars().collect::<HashSet<char>>())
                        .cloned()
                        .collect::<HashSet<char>>();
                }
                chars.len()
            })
            .sum()
    }
}

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let input = buf
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>().concat())
        .collect::<Vec<String>>();
    println!("Part1: {}", Solution::solve(&input));

    let input2 = buf
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    println!("Part2: {}", Solution::solve2(&input2));
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day05() {
        let input_str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        let input = input_str
            .split("\n\n")
            .map(|s| s.split("\n").collect::<Vec<&str>>().concat())
            .collect::<Vec<String>>();

        assert_eq!(11, Solution::solve(&input));

        let input2 = input_str
            .split("\n\n")
            .map(|s| s.split("\n").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        assert_eq!(6, Solution::solve2(&input2));
    }
}
