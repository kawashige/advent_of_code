use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn priority(c: &char) -> usize {
    if c.is_ascii_lowercase() {
        (*c as u8 - b'a' + 1) as usize
    } else {
        (*c as u8 - b'A' + 27) as usize
    }
}

pub fn solve1(rucksacks: &Vec<Vec<char>>) -> usize {
    rucksacks
        .into_iter()
        .map(|rucksack| {
            let first: HashSet<char> = HashSet::from_iter(rucksack[..rucksack.len() / 2].to_vec());
            let second: HashSet<char> = HashSet::from_iter(rucksack[rucksack.len() / 2..].to_vec());
            let common = first.intersection(&second).next().unwrap();
            priority(common)
        })
        .sum::<usize>()
}

pub fn solve2(rucksacks: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let first: HashSet<char> = HashSet::from_iter(rucksacks[i].clone());
        let second: HashSet<char> = HashSet::from_iter(rucksacks[i + 1].clone());
        let third: HashSet<char> = HashSet::from_iter(rucksacks[i + 2].clone());
        let common1 = first.intersection(&second).cloned().collect::<HashSet<_>>();
        let common = common1.intersection(&third).next().unwrap();
        sum += priority(common)
    }
    sum
}

fn main() {
    input! {
        n: usize,
        rucksacks: [Chars; n]
    }

    println!("{:?}", solve1(&rucksacks));
    println!("{:?}", solve2(&rucksacks));
}
