struct Solution {}

impl Solution {
    fn solve(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            if 2020 <= nums[i] {
                break;
            }
            for j in (i + 1)..nums.len() {
                let sum = nums[i] + nums[j];
                if sum == 2020 {
                    return nums[i] * nums[j];
                } else if 2020 < sum {
                    break;
                }
            }
        }
        0
    }

    fn solve2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            if 2020 <= nums[i] {
                break;
            }
            for j in (i + 1)..nums.len() {
                let tmp_sum = nums[i] + nums[j];
                if 2020 <= tmp_sum {
                    break;
                }
                for k in (j + 1)..nums.len() {
                    let sum = tmp_sum + nums[k];
                    if sum == 2020 {
                        return nums[i] * nums[j] * nums[k];
                    } else if 2020 < sum {
                        break;
                    }
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
    let iter = buf.split_whitespace();
    let nums = iter.map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("Part1: {}", Solution::solve(nums.clone()));
    println!("Part2: {}", Solution::solve2(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        assert_eq!(
            1721 * 299,
            Solution::solve(vec![1721, 979, 366, 299, 675, 1456])
        );
        assert_eq!(
            979 * 366 * 675,
            Solution::solve2(vec![1721, 979, 366, 299, 675, 1456])
        );
    }
}
