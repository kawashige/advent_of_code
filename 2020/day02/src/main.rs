struct Input {
    p1: usize,
    p2: usize,
    c: char,
    password: String,
}

struct Solution {}

impl Solution {
    fn valid_count1(inputs: &Vec<Input>) -> usize {
        inputs
            .iter()
            .filter(|i| {
                let count = i.password.chars().filter(|t| t == &i.c).count();
                i.p1 <= count && count <= i.p2
            })
            .count()
    }

    fn valid_count2(inputs: &Vec<Input>) -> usize {
        inputs
            .iter()
            .filter(|i| {
                let chars = i.password.chars().collect::<Vec<char>>();
                (chars[i.p1 - 1] == i.c || chars[i.p2 - 1] == i.c) && chars[i.p1 - 1] != chars[i.p2 - 1]
            })
            .count()
    }

    fn string_to_input(s: String) -> Input {
        let splitted = s.split_whitespace().collect::<Vec<_>>();
        let min_max = splitted[0]
            .split("-")
            .map(|m| m.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let c = splitted[1].chars().next().unwrap();
        Input {
            p1: min_max[0],
            p2: min_max[1],
            c,
            password: splitted[2].to_string(),
        }
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let inputs = buf
        .split("\n")
        .map(|s| Solution::string_to_input(s.to_string()))
        .collect::<Vec<Input>>();
    println!("Part1: {}", Solution::valid_count1(&inputs));
    println!("Part1: {}", Solution::valid_count2(&inputs));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day02() {
        assert_eq!(
            2,
            Solution::valid_count1(&vec![
                Solution::string_to_input("1-3 a: abcde".to_string()),
                Solution::string_to_input("1-3 b: cdefg".to_string()),
                Solution::string_to_input("2-9 c: ccccccccc".to_string())
            ])
        );
        assert_eq!(
            1,
            Solution::valid_count2(&vec![
                Solution::string_to_input("1-3 a: abcde".to_string()),
                Solution::string_to_input("1-3 b: cdefg".to_string()),
                Solution::string_to_input("2-9 c: ccccccccc".to_string())
            ])
        );
    }
}
