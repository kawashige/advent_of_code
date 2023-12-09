use std::io::Read;

fn parse_report(s: String) -> Vec<Vec<i32>> {
    s.split("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn soleve1(report: &[Vec<i32>]) -> i32 {
    report
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];
            while sequences.last().unwrap().iter().any(|num| num != &0) {
                sequences.push(
                    (1..sequences.last().unwrap().len())
                        .map(|i| sequences.last().unwrap()[i] - sequences.last().unwrap()[i - 1])
                        .collect::<Vec<_>>(),
                );
            }
            let mut num = 0;
            for i in (0..sequences.len() - 1).rev() {
                num += sequences[i].last().unwrap();
            }
            num
        })
        .sum()
}

fn soleve2(report: &[Vec<i32>]) -> i32 {
    report
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];
            while sequences.last().unwrap().iter().any(|num| num != &0) {
                sequences.push(
                    (1..sequences.last().unwrap().len())
                        .map(|i| sequences.last().unwrap()[i] - sequences.last().unwrap()[i - 1])
                        .collect::<Vec<_>>(),
                );
            }
            let mut num = 0;
            for i in (0..sequences.len() - 1).rev() {
                num = sequences[i][0] - num;
            }
            num
        })
        .sum()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let report = parse_report(buf);

    println!("{}", soleve1(&report));
    println!("{}", soleve2(&report));
}
