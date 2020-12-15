struct Solution {}

use std::collections::HashMap;
impl Solution {
    fn calc(nums: &Vec<i32>, k: usize) -> i32 {
        let mut map = HashMap::new();
        let mut last = nums[0] as usize;
        for i in 1..k {
            if i < nums.len() {
                map.insert(last, i);
                last = nums[i] as usize;
            } else {
                if let Some(j) = map.get_mut(&last) {
                    last = i - *j;
                    *j = i;
                } else {
                    map.insert(last, i);
                    last = 0;
                }
            }
        }
        last as i32
    }

    fn solve(nums: &Vec<i32>) -> i32 {
        Solution::calc(nums, 2020)
    }

    fn solve2(nums: &Vec<i32>) -> i32 {
        Solution::calc(nums, 30000000)
    }
}

fn main() {
    let nums = "0,1,4,13,15,12,16"
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("Part1: {}", Solution::solve(&nums));
    println!("Part2: {}", Solution::solve2(&nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        assert_eq!(436, Solution::solve(&vec![0, 3, 6]));
        assert_eq!(1836, Solution::solve(&vec![3, 1, 2]));
        assert_eq!(175594, Solution::solve2(&vec![0, 3, 6]));
        assert_eq!(362, Solution::solve2(&vec![3, 1, 2]));
    }
}
