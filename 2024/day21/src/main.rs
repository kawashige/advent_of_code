use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let codes = buf
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&codes));
    println!("{:?}", solve2(&codes));
}

fn numeric_keyapd_pos(c: char) -> (i32, i32) {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn directional_keyapd_pos(c: char) -> (i32, i32) {
    match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    }
}

fn get_numeric_sequences(cur: char, next: char) -> Vec<String> {
    let from = numeric_keyapd_pos(cur);
    let to = numeric_keyapd_pos(next);
    let d = (to.0 - from.0, to.1 - from.1);

    let mut moves = vec![];
    // 上下が先
    if !(from.1 == 0 && to.0 == 3) {
        let mut s1 = String::new();
        for _ in 0..d.0.abs() {
            s1.push(if d.0.signum() < 0 { '^' } else { 'v' });
        }
        for _ in 0..d.1.abs() {
            s1.push(if d.1.signum() < 0 { '<' } else { '>' });
        }
        s1.push('A');
        moves.push(s1);
    }

    // 左右が先
    if !(from.0 == 3 && to.1 == 0) {
        let mut s2 = String::new();
        for _ in 0..d.1.abs() {
            s2.push(if d.1.signum() < 0 { '<' } else { '>' });
        }
        for _ in 0..d.0.abs() {
            s2.push(if d.0.signum() < 0 { '^' } else { 'v' });
        }
        s2.push('A');
        moves.push(s2);
    }

    moves
}

fn get_directional_sequences(cur: char, next: char) -> Vec<String> {
    let from = directional_keyapd_pos(cur);
    let to = directional_keyapd_pos(next);
    let d = (to.0 - from.0, to.1 - from.1);

    let mut moves = vec![];
    // 上下が先
    if !(from == (1, 0) && to.0 == 0) {
        let mut s1 = String::new();
        for _ in 0..d.0.abs() {
            s1.push(if d.0.signum() < 0 { '^' } else { 'v' });
        }
        for _ in 0..d.1.abs() {
            s1.push(if d.1.signum() < 0 { '<' } else { '>' });
        }
        s1.push('A');
        moves.push(s1);
    }

    // 左右が先
    if !(from.0 == 0 && to == (1, 0)) {
        let mut s2 = String::new();
        for _ in 0..d.1.abs() {
            s2.push(if d.1.signum() < 0 { '<' } else { '>' });
        }
        for _ in 0..d.0.abs() {
            s2.push(if d.0.signum() < 0 { '^' } else { 'v' });
        }
        s2.push('A');
        moves.push(s2);
    }

    moves
}

fn get_directional_sequences_min_length(
    code: String,
    count: usize,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if count == 0 {
        return code.len();
    }
    if let Some(c) = memo.get(&(code.clone(), count)) {
        return *c;
    }
    let mut prev = 'A';
    let mut result = 0;
    for i in 0..code.len() {
        let mut min_length = std::usize::MAX;
        for s in get_directional_sequences(prev, code.as_bytes()[i] as char) {
            let l = get_directional_sequences_min_length(s, count - 1, memo);
            min_length = min_length.min(l);
        }
        result += min_length;
        prev = code.as_bytes()[i] as char;
    }
    memo.insert((code, count), result);
    result
}

fn solve1(codes: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    let mut memo = HashMap::new();

    for code in codes {
        let mut prev = 'A';
        let mut len = 0;
        for i in 0..code.len() {
            let mut min_length = std::usize::MAX;
            for s1 in get_numeric_sequences(prev, code[i]) {
                min_length = min_length.min(get_directional_sequences_min_length(s1, 2, &mut memo));
            }
            prev = code[i];
            len += min_length;
        }
        sum += len
            * code[..code.len() - 1]
                .iter()
                .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as usize)
    }

    sum
}

fn solve2(codes: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    let mut memo = HashMap::new();

    for code in codes {
        let mut prev = 'A';
        let mut len = 0;
        for i in 0..code.len() {
            let mut min_length = std::usize::MAX;
            for s1 in get_numeric_sequences(prev, code[i]) {
                min_length =
                    min_length.min(get_directional_sequences_min_length(s1, 25, &mut memo));
            }
            prev = code[i];
            len += min_length;
        }
        sum += len
            * code[..code.len() - 1]
                .iter()
                .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as usize)
    }

    sum
}
