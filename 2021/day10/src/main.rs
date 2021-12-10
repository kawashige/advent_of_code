use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn solve(lines: &Vec<Vec<char>>) -> usize {
    let score = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect::<HashMap<_, _>>();

    lines
        .iter()
        .filter_map(|line| find_first_illegal_character(line))
        .map(|c| score[&c])
        .sum()
}

fn solve2(lines: &Vec<Vec<char>>) -> usize {
    let score = vec![('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut scores = lines
        .iter()
        .filter(|line| find_first_illegal_character(line).is_none())
        .map(|line| {
            line.iter()
                .fold(Vec::new(), |mut stack, c| {
                    if score.contains_key(c) {
                        stack.push(*c);
                    } else {
                        stack.pop();
                    }
                    stack
                })
                .into_iter()
                .rev()
                .map(|c| score[&c])
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn find_first_illegal_character(line: &Vec<char>) -> Option<char> {
    let pair = vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut stack = Vec::new();
    for c in line {
        if pair.contains_key(c) {
            if stack.pop() != Some(pair[c]) {
                return Some(*c);
            }
        } else {
            stack.push(*c);
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        lines: [Chars; n]
    }

    println!("part1: {}", solve(&lines));
    println!("part2: {}", solve2(&lines));
}
