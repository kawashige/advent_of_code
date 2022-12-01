use std::io::Read;

fn solve1(calories: &Vec<Vec<usize>>) -> usize {
    calories
        .into_iter()
        .map(|c| c.into_iter().sum::<usize>())
        .max()
        .unwrap()
}

fn solve2(calories: &Vec<Vec<usize>>) -> usize {
    let mut sum_calories = calories
        .into_iter()
        .map(|c| c.into_iter().sum::<usize>())
        .collect::<Vec<_>>();
    sum_calories.sort_unstable_by(|a, b| b.cmp(&a));
    sum_calories[..3].iter().sum::<usize>()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let calories = buf
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&calories));
    println!("{}", solve2(&calories));
}
