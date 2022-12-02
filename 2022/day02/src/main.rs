use proconio::input;

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    pub fn score(&self) -> usize {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    pub fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}

fn result(h1: &Hand, h2: &Hand) -> Result {
    match (h1, h2) {
        (Hand::Rock, Hand::Scissor) | (Hand::Paper, Hand::Rock) | (Hand::Scissor, Hand::Paper) => {
            Result::Win
        }
        (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissor, Hand::Scissor) => {
            Result::Draw
        }
        _ => Result::Lose,
    }
}

fn count_score(hands: &[(Hand, Hand)]) -> usize {
    hands
        .iter()
        .map(|(h1, h2)| h2.score() + result(h2, h1).score())
        .sum::<usize>()
}

fn solve1(guides: &[(char, char)]) -> usize {
    let hands = guides
        .into_iter()
        .map(|(h1, h2)| {
            (
                match h1 {
                    'A' => Hand::Rock,
                    'B' => Hand::Paper,
                    'C' => Hand::Scissor,
                    _ => unreachable!(),
                },
                match h2 {
                    'X' => Hand::Rock,
                    'Y' => Hand::Paper,
                    'Z' => Hand::Scissor,
                    _ => unreachable!(),
                },
            )
        })
        .collect::<Vec<_>>();
    count_score(&hands)
}

fn solve2(guides: &[(char, char)]) -> usize {
    let hands = guides
        .into_iter()
        .map(|(c1, c2)| {
            let h1 = match c1 {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissor,
                _ => unreachable!(),
            };
            let h2 = match (&h1, c2) {
                (Hand::Rock, 'Y') | (Hand::Paper, 'X') | (Hand::Scissor, 'Z') => Hand::Rock,
                (Hand::Rock, 'Z') | (Hand::Paper, 'Y') | (Hand::Scissor, 'X') => Hand::Paper,
                (Hand::Rock, 'X') | (Hand::Paper, 'Z') | (Hand::Scissor, 'Y') => Hand::Scissor,
                _ => unreachable!(),
            };
            (h1, h2)
        })
        .collect::<Vec<_>>();
    count_score(&hands)
}

fn main() {
    input! {
        n: usize,
        guides: [(char, char); n]
    }

    println!("{}", solve1(&guides));
    println!("{}", solve2(&guides));
}
