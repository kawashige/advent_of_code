use std::collections::HashMap;

use proconio::input;

fn solve(signals: &Vec<(Vec<usize>, Vec<usize>)>) -> usize {
    signals
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|o| [2, 3, 4, 7].contains(&o.count_ones()))
                .count()
        })
        .sum()
}

fn solve2(signals: &Vec<(Vec<usize>, Vec<usize>)>) -> usize {
    signals
        .into_iter()
        .map(|(signal, output)| {
            let mut dict = [0; 10];
            dict[1] = *signal.iter().find(|x| x.count_ones() == 2).unwrap();
            dict[4] = *signal.iter().find(|x| x.count_ones() == 4).unwrap();
            dict[7] = *signal.iter().find(|x| x.count_ones() == 3).unwrap();
            dict[8] = *signal.iter().find(|x| x.count_ones() == 7).unwrap();
            dict[3] = *signal
                .iter()
                .find(|x| x.count_ones() == 5 && **x & dict[7] == dict[7])
                .unwrap();
            dict[5] = *signal
                .iter()
                .find(|x| {
                    x.count_ones() == 5 && (**x & dict[4]).count_ones() == 3 && **x != dict[3]
                })
                .unwrap();
            dict[2] = *signal
                .iter()
                .find(|x| x.count_ones() == 5 && **x != dict[3] && **x != dict[5])
                .unwrap();
            dict[9] = *signal
                .iter()
                .find(|x| x.count_ones() == 6 && **x & dict[3] == dict[3])
                .unwrap();
            dict[6] = *signal
                .iter()
                .find(|x| x.count_ones() == 6 && **x & dict[5] == dict[5] && **x != dict[9])
                .unwrap();
            dict[0] = *signal
                .iter()
                .find(|x| x.count_ones() == 6 && **x != dict[9] && **x != dict[6])
                .unwrap();

            let map = dict
                .iter()
                .enumerate()
                .map(|(i, b)| (*b, i))
                .collect::<HashMap<_, _>>();

            output.iter().fold(0, |acc, b| acc * 10 + map[&b])
        })
        .sum()
}

fn str_to_bit(s: &String) -> usize {
    s.chars().fold(0, |b, c| b | 1 << c as usize - 0x61)
}

fn main() {
    input! {
        n: usize,
        signals: [([String; 10], String, [String; 4]); n]
    }

    let signals = signals
        .into_iter()
        .map(|(signal, _, output)| {
            (
                signal.iter().map(str_to_bit).collect::<Vec<_>>(),
                output.iter().map(str_to_bit).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&signals));
    println!("part2: {}", solve2(&signals));
}
