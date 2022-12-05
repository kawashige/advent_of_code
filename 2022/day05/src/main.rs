use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.parse().ok().unwrap()
}

fn get_top(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>()
}

fn solve1(stacks: &Vec<Vec<char>>, operation: &[(usize, usize, usize)]) -> String {
    let mut stacks = stacks.clone();
    for (amount, stack_from, stack_to) in operation {
        for _ in 0..*amount {
            let c = stacks[stack_from - 1].pop().unwrap();
            stacks[stack_to - 1].push(c);
        }
    }
    get_top(&stacks)
}

fn solve2(stacks: &Vec<Vec<char>>, operation: &[(usize, usize, usize)]) -> String {
    let mut stacks = stacks.clone();
    for (amount, stack_from, stack_to) in operation {
        let len = stacks[stack_from - 1].len();
        let mut moved = stacks[stack_from - 1].split_off(len - amount);
        stacks[stack_to - 1].append(&mut moved);
    }
    get_top(&stacks)
}

fn main() {
    let mut lines = Vec::new();
    loop {
        let line: String = read();
        if line.starts_with(" 1 ") {
            break;
        }
        lines.push(line);
    }
    let stack_count = (lines[0].len() + 1) / 4;
    let mut stacks = vec![vec![]; stack_count];
    for line in lines.into_iter().rev() {
        for i in 0..stack_count {
            if line.as_bytes()[1 + i * 4] != b' ' {
                stacks[i].push(line.as_bytes()[1 + i * 4] as char);
            }
        }
    }

    read::<String>();
    let mut operations = Vec::new();
    loop {
        let operation: String = read();
        if operation.is_empty() {
            break;
        }
        let sp = operation
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();
        operations.push((sp[0], sp[1], sp[2]));
    }

    println!("{}", solve1(&stacks, &operations));
    println!("{}", solve2(&stacks, &operations));
}
