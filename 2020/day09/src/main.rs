struct Solution {}

impl Solution {
    fn solve(nums: &Vec<i64>, n: usize) -> i64 {
        for i in n..nums.len() {
            let mut found = false;
            for j in (i - n)..i {
                let tmp = nums[i] - nums[j];
                if tmp < 0 {
                    continue;
                }
                for k in (j + 1)..i {
                    if nums[k] == tmp {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                return nums[i];
            }
        }
        0
    }

    fn solve2(nums: &Vec<i64>, target: i64) -> i64 {
        let mut s = nums[0];
        let mut min_i = 0;
        for i in 1..nums.len() {
            s += nums[i];
            while target < s && min_i < i {
                s -= nums[min_i];
                min_i += 1;
                if s == target && 1 < i - min_i + 1 {
                    let mut v = nums[min_i..=i].iter().collect::<Vec<&i64>>();
                    v.sort();
                    return v[0] + *v.last().unwrap();
                }
            }
        }
        0
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let nums = buf
        .split('\n')
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let r = Solution::solve(&nums, 25);
    println!("Part1: {}", r);
    println!("Part2: {}", Solution::solve2(&nums, r));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let input_str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        let input = input_str
            .split('\n')
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let r = Solution::solve(&input, 5);
        assert_eq!(127, Solution::solve(&input, 5));
        assert_eq!(62, Solution::solve2(&input, r));
    }
}
