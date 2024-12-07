use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let equations = buf
        .split("\n")
        .map(|line| {
            let mut sp = line.split(": ");
            let test_value = sp.next().unwrap().parse::<usize>().unwrap();
            let numbers = sp
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (test_value, numbers)
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&equations));
    println!("{:?}", solve2(&equations));
}

pub fn recurse(i: usize, val: usize, test_value: usize, numbers: &Vec<usize>) -> bool {
    if i == numbers.len() {
        return val == test_value;
    }
    recurse(i + 1, val + numbers[i], test_value, numbers)
        || recurse(i + 1, val * numbers[i], test_value, numbers)
}

pub fn recurse2(i: usize, val: usize, test_value: usize, numbers: &Vec<usize>) -> bool {
    if i == numbers.len() {
        return val == test_value;
    }
    recurse2(i + 1, val + numbers[i], test_value, numbers)
        || recurse2(i + 1, val * numbers[i], test_value, numbers)
        || recurse2(
            i + 1,
            format!("{}{}", val, numbers[i]).parse::<usize>().unwrap(),
            test_value,
            numbers,
        )
}

fn solve1(equations: &Vec<(usize, Vec<usize>)>) -> usize {
    equations
        .iter()
        .map(|e| {
            if recurse(1, e.1[0], e.0, &e.1) {
                e.0
            } else {
                0
            }
        })
        .sum()
}

fn solve2(equations: &Vec<(usize, Vec<usize>)>) -> usize {
    equations
        .iter()
        .map(|e| {
            if recurse2(1, e.1[0], e.0, &e.1) {
                e.0
            } else {
                0
            }
        })
        .sum()
}
