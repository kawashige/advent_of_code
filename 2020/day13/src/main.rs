struct Solution {}

impl Solution {
    fn solve(timestamp: i32, ids: &Vec<i32>) -> i32 {
        let result = ids
            .iter()
            .map(|i| {
                let n = (0..).skip_while(|j| i * j < timestamp).next().unwrap();
                (i, i * n)
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        result.0 * (result.1 - timestamp)
    }

    fn solve2(ids: &Vec<i32>) -> i64 {
        let mut t = ids[0] as i64;
        let mut step = ids[0] as i64;
        for i in 1..ids.len() {
            if ids[i] == 0 {
                continue;
            }
            while (t + i as i64) % ids[i] as i64 != 0 {
                t += step;
            }
            step *= ids[i] as i64;
        }
        t
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.split('\n');
    let timestamp = lines.next().unwrap().parse::<i32>().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    let ids1 = ids
        .iter()
        .cloned()
        .filter(|i| i != &0)
        .collect::<Vec<i32>>();
    println!("Part1: {}", Solution::solve(timestamp, &ids1));
    println!("Part2: {}", Solution::solve2(&ids));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str1 = r#"939
7,13,x,x,59,x,31,19"#;

        let mut lines = str1.split('\n');
        let timestamp = lines.next().unwrap().parse::<i32>().unwrap();
        let ids = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();
        let ids1 = ids
            .iter()
            .cloned()
            .filter(|i| i != &0)
            .collect::<Vec<i32>>();

        assert_eq!(295, Solution::solve(timestamp, &ids1));
        assert_eq!(1068781, Solution::solve2(&ids));
        assert_eq!(1261476, Solution::solve2(&vec![67, 7, 0, 59, 61]));
        assert_eq!(1202161486, Solution::solve2(&vec![1789, 37, 47, 1889]));
    }
}
