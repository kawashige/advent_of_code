struct Solution {}

impl Solution {
    fn solve(map: &Vec<Vec<char>>) -> usize {
        let mut result = 0;
        let mut j = 3 % map[0].len();
        for i in 1..map.len() {
            if map[i][j] == '#' {
                result += 1;
            }
            j = (j + 3) % map[0].len();
        }
        result
    }

    fn solve2(map: &Vec<Vec<char>>) -> usize {
        let slopes = [1, 3, 5, 7, 1];
        let mut results = vec![0, 0, 0, 0, 0];
        let mut nexts = slopes
            .iter()
            .map(|s| s % map[0].len())
            .collect::<Vec<usize>>();

        for i in 1..map.len() {
            for j in 0..(nexts.len() - 1) {
                if map[i][nexts[j]] == '#' {
                    results[j] += 1;
                }
                nexts[j] = (nexts[j] + slopes[j]) % map[0].len();
            }
            if i % 2 == 0 {
                let k = nexts.len() - 1;
                if map[i][nexts[k]] == '#' {
                    results[k] += 1;
                }
                nexts[k] = (nexts[k] + slopes[k]) % map[0].len();
            }
        }
        results.into_iter().product()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let chars = buf
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Part1: {}", Solution::solve(&chars));
    println!("Part2: {}", Solution::solve2(&chars));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let input = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

        assert_eq!(7, Solution::solve(&input));
        assert_eq!(336, Solution::solve2(&input));
    }
}
