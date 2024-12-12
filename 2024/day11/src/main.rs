use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let stones = buf
        .split_ascii_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&stones));
    println!("{:?}", solve2(&stones));
}

pub fn change(stone: usize, times: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(count) = memo.get(&(stone, times)) {
        return *count;
    }
    if times == 0 {
        return 1;
    }
    let count = if stone == 0 {
        change(1, times - 1, memo)
    } else if stone.to_string().len() % 2 == 0 {
        let x = 10_usize.pow(stone.to_string().len() as u32 / 2);
        change(stone % x, times - 1, memo) + change(stone / x, times - 1, memo)
    } else {
        change(stone * 2024, times - 1, memo)
    };
    memo.insert((stone, times), count);
    count
}

fn solve1(stones: &Vec<usize>) -> usize {
    stones
        .iter()
        .map(|stone| change(*stone, 25, &mut HashMap::new()))
        .sum()
}

fn solve2(stones: &Vec<usize>) -> usize {
    stones
        .iter()
        .map(|stone| change(*stone, 75, &mut HashMap::new()))
        .sum()
}
