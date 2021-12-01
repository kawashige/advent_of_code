use proconio::input;

fn solve(report: &Vec<usize>) -> usize {
    report.windows(2).filter(|r| r[0] < r[1]).count()
}

fn solve2(report: &Vec<usize>) -> usize {
    solve(
        &report
            .windows(3)
            .map(|r| r.into_iter().sum())
            .collect::<Vec<usize>>(),
    )
}
fn main() {
    input! {
        n: usize,
        report: [usize; n]
    }

    println!("part1: {}", solve(&report));
    println!("part2: {}", solve2(&report));
}
