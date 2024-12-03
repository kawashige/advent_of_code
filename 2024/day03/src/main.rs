use regex::Regex;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    println!("{:?}", solve1(&buf));
    println!("{:?}", solve2(&buf));
}

fn solve1(memory: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;

    for caps in re.captures_iter(memory) {
        let num1 = caps[1].parse::<usize>().unwrap();
        let num2 = caps[2].parse::<usize>().unwrap();
        result += num1 * num2;
    }

    result
}

fn solve2(memory: &str) -> usize {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut l = 0;

    while l < memory.len() {
        let r = (l..memory.len())
            .find(|i| memory[*i..].starts_with("don't()"))
            .unwrap_or(memory.len());

        for caps in re.captures_iter(&memory[l..r]) {
            let num1 = caps[1].parse::<usize>().unwrap();
            let num2 = caps[2].parse::<usize>().unwrap();
            result += num1 * num2;
        }

        l = r + 7;
        if l < memory.len() {
            if let Some(i) = (l..memory.len()).find(|i| memory[*i..].starts_with("do()")) {
                l = i + 4;
            } else {
                break;
            }
        }
    }

    result
}
