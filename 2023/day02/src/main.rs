use std::{io::Read, u128::MAX};

fn color_index(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => unreachable!(),
    }
}

fn parse_games(s: String) -> Vec<(usize, Vec<[usize; 3]>)> {
    s.split("\n")
        .map(|game| {
            let mut sp = game.split(":");
            let id = sp
                .next()
                .unwrap()
                .trim_start_matches("Game ")
                .parse::<usize>()
                .unwrap();
            let sets = sp
                .next()
                .unwrap()
                .split(";")
                .map(|set| {
                    set.split(", ").fold([0; 3], |mut counts, cubes| {
                        let mut sp = cubes.split_whitespace();
                        let count = sp.next().unwrap().parse::<usize>().unwrap();
                        let color = sp.next().unwrap();
                        counts[color_index(color)] += count;
                        counts
                    })
                })
                .collect::<Vec<_>>();
            (id, sets)
        })
        .collect()
}

fn solve1(games: &[(usize, Vec<[usize; 3]>)]) -> usize {
    const MAX_COUNTS: [usize; 3] = [12, 13, 14];
    games
        .iter()
        .filter(|(_, sets)| {
            sets.iter()
                .all(|set| MAX_COUNTS.iter().zip(set.iter()).all(|(c1, c2)| c2 <= c1))
        })
        .map(|(id, _)| id)
        .sum()
}

fn solve2(games: &[(usize, Vec<[usize; 3]>)]) -> usize {
    games
        .iter()
        .map(|(_, sets)| {
            let mut counts = [0; 3];
            for i in 0..sets.len() {
                for j in 0..counts.len() {
                    counts[j] = counts[j].max(sets[i][j]);
                }
            }
            counts
        })
        .map(|counts| counts.iter().fold(1, |acc, c| acc * c))
        .sum()
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let games = parse_games(buf);

    println!("{}", solve1(&games));
    println!("{}", solve2(&games));
}
