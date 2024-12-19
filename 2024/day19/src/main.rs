use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (towels, designs) = {
        let mut sp = buf.split("\n\n");
        let towels = sp
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let designs = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        (towels, designs)
    };

    println!("{:?}", solve1(&towels, &designs));
    println!("{:?}", solve2(&towels, &designs));
}

fn recurse(i: usize, design: &str, towels: &Vec<String>) -> bool {
    if i == design.len() {
        return true;
    }

    for j in 0..towels.len() {
        if design[i..].starts_with(&towels[j]) && recurse(i + towels[j].len(), design, towels) {
            return true;
        }
    }

    false
}

fn solve1(towels: &Vec<String>, designs: &Vec<String>) -> usize {
    designs
        .iter()
        .filter(|design| recurse(0, *design, towels))
        .count()
}

fn recurse2(
    i: usize,
    design: &str,
    towels: &Vec<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if i == design.len() {
        return 1;
    }
    if let Some(count) = memo.get(&design[i..]) {
        return *count;
    }
    let mut count = 0;

    for j in 0..towels.len() {
        if design[i..].starts_with(&towels[j]) {
            count += recurse2(i + towels[j].len(), design, towels, memo);
        }
    }

    memo.insert(design[i..].to_string(), count);
    count
}

fn solve2(towels: &Vec<String>, designs: &Vec<String>) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| recurse2(0, design, towels, &mut memo))
        .sum()
}
