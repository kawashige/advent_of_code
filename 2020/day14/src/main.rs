enum Ops {
    Mask(i64, i64, i64),
    Mem((i64, i64)),
}

struct Solution {}

use regex::Regex;
use std::collections::HashMap;
impl Solution {
    fn solve(ops: &Vec<Ops>) -> i64 {
        let mut mask_one = 1;
        let mut mask_zeno = 0;
        let mut map = HashMap::new();
        for op in ops {
            match op {
                Ops::Mask(one, zero, _) => {
                    mask_one = *one;
                    mask_zeno = *zero;
                }
                Ops::Mem((address, val)) => {
                    map.insert(address, (val | mask_one) & mask_zeno);
                }
            }
        }
        map.values().into_iter().sum()
    }

    fn solve2(ops: &Vec<Ops>) -> i64 {
        let mut mask_one = 1;
        let mut mask_x = 0;
        let mut map = HashMap::new();
        for op in ops {
            match op {
                Ops::Mask(one, _, x) => {
                    mask_one = *one;
                    mask_x = *x;
                }
                Ops::Mem((address, val)) => {
                    let mut addresses = vec![address | mask_one];
                    for i in 0..36 {
                        if 0 < mask_x & (1 << i) {
                            addresses = addresses
                                .into_iter()
                                .map(|a| vec![(a | (1 << i)), (a & !(1 << i))])
                                .flatten()
                                .collect::<Vec<i64>>()
                        }
                    }
                    for a in addresses {
                        map.insert(a, *val);
                    }
                }
            }
        }
        map.values().into_iter().sum()
    }

    fn str_to_input(s: String) -> Vec<Ops> {
        let re = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap();
        s.split('\n')
            .map(|s| {
                if s.starts_with("mask = ") {
                    let mask = s.strip_prefix("mask = ").unwrap().to_string();
                    Ops::Mask(
                        i64::from_str_radix(&mask.replace("X", "0"), 2).unwrap(),
                        i64::from_str_radix(&mask.replace("X", "1"), 2).unwrap(),
                        i64::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap(),
                    )
                } else {
                    let caps = re.captures(s).unwrap();
                    Ops::Mem((
                        caps.get(1)
                            .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
                        caps.get(2)
                            .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
                    ))
                }
            })
            .collect::<Vec<Ops>>()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let input = Solution::str_to_input(buf);
    println!("Part1: {}", Solution::solve(&input));
    println!("Part2: {}", Solution::solve2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01() {
        let str1 = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let input = Solution::str_to_input(str1.to_string());
        assert_eq!(165, Solution::solve(&input));

        let str1 = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        let input = Solution::str_to_input(str1.to_string());
        assert_eq!(208, Solution::solve2(&input));
    }
}
