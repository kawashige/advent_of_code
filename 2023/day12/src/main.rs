use std::{collections::HashMap, io::Read};

fn parse_records(s: String) -> Vec<(Vec<char>, Vec<usize>)> {
    s.split("\n")
        .map(|line| {
            let mut sp = line.split_ascii_whitespace();
            let records = sp.next().unwrap().chars().collect();
            let group = sp
                .next()
                .unwrap()
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            (records, group)
        })
        .collect()
}

fn recurse(
    i: usize,
    j: usize,
    record: &Vec<char>,
    group: &Vec<usize>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if j == group.len() {
        return if record.len() <= i || record[i..].iter().all(|c| c != &'#') {
            1
        } else {
            0
        };
    }
    if record.len() <= i || record.len() < i + group[j] {
        return 0;
    }

    if let Some(count) = memo.get(&(i, j)) {
        return *count;
    }

    let mut count = 0;
    if record[i..i + group[j]].iter().all(|c| c != &'.')
        && (i + group[j] == record.len() || record[i + group[j]] != '#')
    {
        count += recurse(i + group[j] + 1, j + 1, record, group, memo)
    }
    if record[i] != '#' {
        count += recurse(i + 1, j, record, group, memo)
    }
    memo.insert((i, j), count);
    count
}

fn solve1(records: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    records
        .iter()
        .map(|(record, group)| recurse(0, 0, record, group, &mut HashMap::new()))
        .sum()
}

fn solve2(records: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    let mut records = records.clone();
    records = records
        .into_iter()
        .map(|(record, group)| {
            let len = record.len() * 5 + 4;
            let record = record
                .into_iter()
                .chain(std::iter::once('?'))
                .cycle()
                .take(len)
                .collect::<Vec<_>>();
            let len = group.len() * 5;
            let group = group.into_iter().cycle().take(len).collect::<Vec<_>>();
            (record, group)
        })
        .collect::<Vec<_>>();
    records
        .iter()
        .map(|(record, group)| recurse(0, 0, record, group, &mut HashMap::new()))
        .sum()
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let records = parse_records(buf);

    println!("{}", solve1(&records));
    println!("{}", solve2(&records));
}
