struct Solution {}

use std::collections::HashSet;
impl Solution {
    fn solve(ops: &Vec<Vec<&str>>) -> i32 {
        let mut acc = 0;
        let mut i = 0;
        let mut opened = HashSet::new();
        while !opened.contains(&i) {
            opened.insert(i);
            match ops[i][0] {
                "acc" => {
                    acc += ops[i][1].parse::<i32>().unwrap();
                    i += 1;
                }
                "jmp" => {
                    i = (i as i32 + ops[i][1].parse::<i32>().unwrap()) as usize;
                }
                _ => {
                    i += 1;
                }
            }
        }

        acc
    }

    fn solve2(ops: &Vec<Vec<&str>>) -> i32 {
        fn recurse(
            ops: &Vec<Vec<&str>>,
            i: usize,
            opened: &mut HashSet<usize>,
            acc: i32,
            changed: bool,
        ) -> Option<i32> {
            if ops.len() <= i {
                return Some(acc);
            }
            if opened.contains(&i) {
                return None;
            }
            opened.insert(i);
            match ops[i][0] {
                "acc" => recurse(
                    ops,
                    i + 1,
                    opened,
                    acc + ops[i][1].parse::<i32>().unwrap(),
                    changed,
                ),
                _ => {
                    let mut r = None;
                    if ops[i][0] == "jmp" || !changed {
                        r = recurse(
                            ops,
                            (i as i32 + ops[i][1].parse::<i32>().unwrap()) as usize,
                            &mut opened.clone(),
                            acc,
                            if ops[i][0] == "jmp" { changed } else { true },
                        );
                    }
                    if r.is_none() && (ops[i][0] == "nop" || !changed) {
                        r = recurse(
                            ops,
                            i + 1,
                            &mut opened.clone(),
                            acc,
                            if ops[i][0] == "nop" { changed } else { true },
                        );
                    }
                    r
                }
            }
        }
        recurse(ops, 0, &mut HashSet::new(), 0, false).unwrap()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let ops = buf
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<_>>>();
    println!("Part1: {}", Solution::solve(&ops));
    println!("Part2: {}", Solution::solve2(&ops));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day08() {
        let str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let input = str
            .split("\n")
            .map(|s| s.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<_>>>();

        assert_eq!(5, Solution::solve(&input));
        assert_eq!(8, Solution::solve2(&input));
    }
}
