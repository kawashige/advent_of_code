use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let reports = buf
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&reports));
    println!("{:?}", solve2(&reports));
}

fn check(report: &[i32]) -> bool {
    let mut sign = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let new_sign = (report[i] - report[i - 1]).signum();
        if !(1..4).contains(&(report[i] - report[i - 1]).abs()) || sign != new_sign {
            return false;
        }
        sign = new_sign;
    }
    true
}

fn solve1(reports: &Vec<Vec<i32>>) -> i32 {
    reports.into_iter().filter(|report| check(&report)).count() as i32
}

fn solve2(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .into_iter()
        .filter(|report| {
            let mut lists = vec![report.to_vec()];
            for i in 0..report.len() {
                let mut new_report = report.to_vec();
                new_report.remove(i);
                lists.push(new_report);
            }
            lists.into_iter().any(|l| check(&l))
        })
        .count() as i32
}
