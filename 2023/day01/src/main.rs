use std::collections::HashMap;

use proconio::input;

fn solve1(calibration: &[String]) -> usize {
    calibration
        .iter()
        .map(|values| {
            let digits = values
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum::<u32>() as usize
}

fn solve2(calibration: &[String]) -> usize {
    let digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    calibration
        .iter()
        .map(|values| {
            let mut d = digits
                .keys()
                .flat_map(|d| values.match_indices(d).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            d.sort_unstable();
            digits[d[0].1] * 10 + digits[d[d.len() - 1].1]
        })
        .sum::<u32>() as usize
}

fn main() {
    input! {
        n: usize,
        calibration: [String; n]
    }

    // println!("{:?}", solve1(&calibration));
    println!("{:?}", solve2(&calibration));
}
