struct Solution {
    expressions: Vec<String>,
}

impl Solution {
    fn new(s: String) -> Self {
        let expressions = s
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self { expressions }
    }

    fn calc(s: &str) -> i64 {
        let mut num_stack = vec![vec![]];
        let mut op_stack = vec![vec![]];
        for c in s.chars() {
            match c {
                ' ' => {}
                '+' | '*' => {
                    op_stack.last_mut().unwrap().push(c);
                }
                '(' => {
                    num_stack.push(Vec::new());
                    op_stack.push(Vec::new());
                }
                ')' => {
                    let nums = num_stack.pop().unwrap();
                    let ops = op_stack.pop().unwrap();
                    let mut tmp = nums[0];
                    for i in 1..nums.len() {
                        if ops[i - 1] == '+' {
                            tmp += nums[i];
                        } else {
                            tmp *= nums[i];
                        }
                    }
                    num_stack.last_mut().unwrap().push(tmp);
                }
                _ => {
                    num_stack
                        .last_mut()
                        .unwrap()
                        .push(c.to_digit(10).unwrap() as i64);
                }
            }
        }
        let nums = num_stack.pop().unwrap();
        let ops = op_stack.pop().unwrap();
        let mut result = nums[0];
        for i in 1..nums.len() {
            if ops[i - 1] == '+' {
                result += nums[i];
            } else {
                result *= nums[i];
            }
        }
        result
    }

    fn calc2(s: &str) -> i64 {
        let mut num_stack = vec![vec![]];
        let mut op_stack = vec![vec![]];
        for c in s.chars() {
            match c {
                ' ' => {}
                '+' | '*' => {
                    op_stack.last_mut().unwrap().push(c);
                }
                '(' => {
                    num_stack.push(Vec::new());
                    op_stack.push(Vec::new());
                }
                ')' => {
                    let mut nums = num_stack.pop().unwrap();
                    let mut ops = op_stack.pop().unwrap();

                    while ops.contains(&'+') {
                        let pos = (0..ops.len()).find(|i| ops[*i] == '+').unwrap();
                        nums[pos] += nums[pos + 1];
                        nums.remove(pos + 1);
                        ops.remove(pos);
                    }
                    num_stack.last_mut().unwrap().push(nums.iter().product());
                }
                _ => {
                    num_stack
                        .last_mut()
                        .unwrap()
                        .push(c.to_digit(10).unwrap() as i64);
                }
            }
        }
        let mut nums = num_stack.pop().unwrap();
        let mut ops = op_stack.pop().unwrap();

        while ops.contains(&'+') {
            let pos = (0..ops.len()).find(|i| ops[*i] == '+').unwrap();
            nums[pos] += nums[pos + 1];
            nums.remove(pos + 1);
            ops.remove(pos);
        }
        nums.iter().product()
    }

    fn solve(&self) -> i64 {
        self.expressions.iter().map(|s| Self::calc(s)).sum()
    }

    fn solve2(&self) -> i64 {
        self.expressions.iter().map(|s| Self::calc2(s)).sum()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let solution = Solution::new(buf);
    println!("Part1: {}", solution.solve());
    println!("Part2: {}", solution.solve2());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let solution = Solution::new("2 * 3 + (4 * 5)".to_string());
        assert_eq!(26, solution.solve());

        let solution = Solution::new("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string());
        assert_eq!(437, solution.solve());

        let solution = Solution::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string());
        assert_eq!(12240, solution.solve());

        let solution =
            Solution::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ".to_string());
        assert_eq!(13632, solution.solve());

        let solution = Solution::new("1 + (2 * 3) + (4 * (5 + 6))".to_string());
        assert_eq!(51, solution.solve2());

        let solution = Solution::new("2 * 3 + (4 * 5)".to_string());
        assert_eq!(46, solution.solve2());

        let solution = Solution::new("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string());
        assert_eq!(1445, solution.solve2());

        let solution = Solution::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string());
        assert_eq!(669060, solution.solve2());

        let solution = Solution::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string());
        assert_eq!(23340, solution.solve2());
    }
}
