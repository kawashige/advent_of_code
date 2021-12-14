use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn solve(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    simulate(template, rules, 10)
}

fn solve2(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    simulate(template, rules, 40)
}

fn simulate(template: &Vec<char>, rules: &HashMap<(char, char), char>, iteration: usize) -> usize {
    let mut polymer = (1..template.len()).fold(HashMap::new(), |mut map, i| {
        *map.entry((template[i - 1], template[i])).or_insert(0) += 1;
        map
    });

    for _ in 0..iteration {
        polymer = insertion(&polymer, rules);
    }

    let mut counts = [0; 26];
    counts[template[0] as usize - 0x41] += 1;
    for ((_, c), count) in polymer {
        counts[c as usize - 0x41] += count;
    }
    counts.iter().max().unwrap() - counts.iter().filter(|c| c > &&0).min().unwrap()
}

fn insertion(
    polymer: &HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    polymer
        .iter()
        .fold(HashMap::new(), |mut map, ((c1, c2), count)| {
            let c_i = rules[&(*c1, *c2)];
            *map.entry((*c1, c_i)).or_insert(0) += count;
            *map.entry((c_i, *c2)).or_insert(0) += count;
            map
        })
}

fn main() {
    input! {
        template: Chars,
        n: usize,
        rules: [[String; 3]; n]
    }

    let rules = rules.into_iter().fold(
        HashMap::new(),
        |mut map: HashMap<(char, char), char>, rule| {
            let mut key_chars = rule[0].chars();
            map.insert(
                (key_chars.next().unwrap(), key_chars.next().unwrap()),
                rule[2].chars().next().unwrap(),
            );
            map
        },
    );

    println!("part1: {}", solve(&template, &rules));
    println!("part2: {}", solve2(&template, &rules));
}
