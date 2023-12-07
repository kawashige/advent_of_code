use proconio::input;
use proconio::marker::Chars;

fn strength(card: &char) -> usize {
    match card {
        &'A' => 0,
        &'K' => 1,
        &'Q' => 2,
        &'J' => 3,
        &'T' => 4,
        &'9' => 5,
        &'8' => 6,
        &'7' => 7,
        &'6' => 8,
        &'5' => 9,
        &'4' => 10,
        &'3' => 11,
        &'2' => 12,
        _ => unreachable!(),
    }
}
fn strength2(card: &char) -> usize {
    match card {
        &'A' => 0,
        &'K' => 1,
        &'Q' => 2,
        &'T' => 3,
        &'9' => 4,
        &'8' => 5,
        &'7' => 6,
        &'6' => 7,
        &'5' => 8,
        &'4' => 9,
        &'3' => 10,
        &'2' => 11,
        &'J' => 12,
        _ => unreachable!(),
    }
}

fn get_type(hand: &[char]) -> usize {
    let mut count = vec![0; 13];
    for c in hand {
        count[strength(c)] += 1;
    }
    let mut count = count.into_iter().filter(|c| &0 < c).collect::<Vec<_>>();
    count.sort_unstable();
    match &count[..] {
        [5] => 0,
        [1, 4] => 1,
        [2, 3] => 2,
        [1, 1, 3] => 3,
        [1, 2, 2] => 4,
        [1, 1, 1, 2] => 5,
        _ => 6,
    }
}

fn get_type2(hand: &[char]) -> usize {
    let mut count = vec![0; 13];
    for c in hand {
        count[strength2(c)] += 1;
    }
    let j = strength2(&'J');
    if 0 < count[j] {
        let i = (0..count.len())
            .filter(|i| i != &j)
            .max_by_key(|i| count[*i])
            .unwrap();
        if 0 < count[i] {
            count[i] += count[j];
            count[j] = 0;
        }
    }
    let mut count = count.into_iter().filter(|c| &0 < c).collect::<Vec<_>>();
    count.sort_unstable();
    match &count[..] {
        [5] => 0,
        [1, 4] => 1,
        [2, 3] => 2,
        [1, 1, 3] => 3,
        [1, 2, 2] => 4,
        [1, 1, 1, 2] => 5,
        _ => 6,
    }
}

fn get_hand_strength(hand: &[char]) -> Vec<usize> {
    hand.iter().map(|card| strength(card)).collect::<Vec<_>>()
}

fn get_hand_strength2(hand: &[char]) -> Vec<usize> {
    hand.iter().map(|card| strength2(card)).collect::<Vec<_>>()
}
fn solve1(hands: &[(Vec<char>, usize)]) -> usize {
    let mut hands = hands.to_vec();
    hands.sort_unstable_by(|a, b| {
        get_type(&a.0)
            .cmp(&get_type(&b.0))
            .then(get_hand_strength(&a.0).cmp(&get_hand_strength(&b.0)))
    });

    hands
        .into_iter()
        .rev()
        .zip(1..)
        .map(|(hands, i)| hands.1 * i)
        .sum::<usize>()
}

fn solve2(hands: &[(Vec<char>, usize)]) -> usize {
    let mut hands = hands.to_vec();
    hands.sort_unstable_by(|a, b| {
        get_type2(&a.0)
            .cmp(&get_type2(&b.0))
            .then(get_hand_strength2(&a.0).cmp(&get_hand_strength2(&b.0)))
    });

    hands
        .into_iter()
        .rev()
        .zip(1..)
        .map(|(hands, i)| hands.1 * i)
        .sum::<usize>()
}
fn main() {
    input! {
        n: usize,
        hands: [(Chars, usize); n]
    }

    println!("{}", solve1(&hands));
    println!("{}", solve2(&hands));
}
