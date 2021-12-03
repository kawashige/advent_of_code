use proconio::input;

fn solve(report: &[String]) -> usize {
    let (gamma, epsilon) =
        count_one(report)
            .iter()
            .rev()
            .enumerate()
            .fold((0, 0), |(g, e), (i, c)| {
                if *c > report.len() / 2 {
                    (g | 1 << i, e)
                } else {
                    (g, e | 1 << i)
                }
            });

    gamma * epsilon
}

fn solve2(report: &[String]) -> usize {
    let oxygen_generator_rating = get_rating(report, '1');
    let co2_scrubber_rating = get_rating(report, '0');

    oxygen_generator_rating * co2_scrubber_rating
}

fn count_one(report: &[String]) -> Vec<usize> {
    report.iter().fold(vec![0; report[0].len()], |count, r| {
        r.chars()
            .zip(count.iter())
            .map(|(c1, c2)| if c1 == '0' { 0 } else { 1 } + c2)
            .collect::<Vec<_>>()
    })
}

fn get_rating(report: &[String], target: char) -> usize {
    let mut remains = report.to_vec();
    for i in 0..report[0].len() {
        let c = remains
            .iter()
            .filter(|r| r.as_bytes()[i] == '1' as u8)
            .count();
        let l = remains.len();
        remains = remains
            .into_iter()
            .filter(|r| {
                if c * 2 >= l {
                    r.as_bytes()[i] == target as u8
                } else {
                    r.as_bytes()[i] != target as u8
                }
            })
            .collect::<Vec<_>>();
        if remains.len() == 1 {
            break;
        }
    }
    usize::from_str_radix(&remains.pop().unwrap(), 2).unwrap()
}

fn main() {
    input! {
        n: usize,
        report: [String; n]
    }

    println!("part1: {}", solve(&report));
    println!("part2: {}", solve2(&report));
}
