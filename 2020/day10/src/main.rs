struct Solution {}

impl Solution {
    fn solve(nums: &Vec<i32>) -> i32 {
        let mut current = 0;
        let mut diff_1 = 0;
        let mut diff_3 = 1;
        for n in nums {
            if n - current == 1 {
                diff_1 += 1;
            } else if n - current == 3 {
                diff_3 += 1;
            }
            current = *n;
        }
        diff_1 * diff_3
    }

    fn solve2(nums: &Vec<i32>) -> i64 {
        let mut dp = vec![0; nums.len()];
        for i in 0..nums.len() {
            let mut count = if nums[i] < 4 { 1 } else { 0 };
            let min = if i < 3 { 0 } else { i - 3 };
            for j in (min..i).rev() {
                if nums[i] - nums[j] <= 3 {
                    count += dp[j];
                } else {
                    break;
                }
            }
            dp[i] = count;
        }
        *dp.last().unwrap()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut nums = buf
        .split('\n')
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    println!("Part1: {}", Solution::solve(&nums));
    println!("Part2: {}", Solution::solve2(&nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str1 = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        let mut input1 = str1
            .split('\n')
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        input1.sort();

        assert_eq!(35, Solution::solve(&input1));
        assert_eq!(8, Solution::solve2(&input1));

        let str2 = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        let mut input2 = str2
            .split('\n')
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        input2.sort();

        assert_eq!(220, Solution::solve(&input2));
        assert_eq!(19208, Solution::solve2(&input2));
    }
}
