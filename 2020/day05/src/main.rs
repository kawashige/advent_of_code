struct Solution {}

impl Solution {
    fn id(s: &str) -> usize {
        let mut row_low = 0;
        let mut row_high = 127;
        for c in s[..7].chars() {
            if c == 'F' {
                row_high -= (row_high - row_low + 1) / 2
            } else {
                row_low += (row_high - row_low + 1) / 2
            }
        }
        let result = row_low * 8;

        let mut col_low = 0;
        let mut col_high = 7;
        for c in s[7..].chars() {
            if c == 'L' {
                col_high -= (col_high - col_low + 1) / 2
            } else {
                col_low += (col_high - col_low + 1) / 2
            }
        }
        result + col_low
    }

    fn solve(passes: &Vec<&str>) -> usize {
        passes.iter().map(|p| Solution::id(*p)).max().unwrap()
    }

    fn solve2(passes: &Vec<&str>) -> usize {
        let mut ids = passes
            .iter()
            .map(|p| Solution::id(*p))
            .collect::<Vec<usize>>();
        ids.sort();
        for i in 0..(ids.len() - 1) {
            if ids[i + 1] - ids[i] == 2 {
                return ids[i] + 1;
            }
        }
        0
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let input = buf.split("\n").collect::<Vec<&str>>();
    println!("Part1: {}", Solution::solve(&input));
    println!("Part2: {}", Solution::solve2(&input));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day05() {
        assert_eq!(567, Solution::id("BFFFBBFRRR"));
        assert_eq!(119, Solution::id("FFFBBBFRRR"));
        assert_eq!(820, Solution::id("BBFFBBFRLL"));
    }
}
