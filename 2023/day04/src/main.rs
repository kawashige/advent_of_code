use std::io::Read;

fn parse_cards(s: String) -> Vec<(usize, Vec<usize>, Vec<usize>)> {
    s.split('\n')
        .enumerate()
        .map(|(i, card)| {
            let mut sp = card.split(&[':', '|']);
            sp.next();
            let winning_numbers = sp
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let numbers = sp
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (i + 1, winning_numbers, numbers)
        })
        .collect()
}

fn solve1(cards: &[(usize, Vec<usize>, Vec<usize>)]) -> usize {
    cards
        .iter()
        .map(|(_, winning_numbers, numbers)| {
            numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count()
        })
        .map(|c| if c == 0 { 0 } else { 2_usize.pow(c as u32 - 1) })
        .sum()
}

fn solve2(cards: &[(usize, Vec<usize>, Vec<usize>)]) -> usize {
    let mut instances = vec![1; cards.len()];

    for i in 0..cards.len() {
        let matching_numbers = cards[i].2.iter().filter(|n| cards[i].1.contains(n)).count();
        for j in 0..matching_numbers {
            if cards.len() <= i + 1 + j {
                break;
            }
            instances[i + 1 + j] += instances[i];
        }
    }

    instances.into_iter().sum()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let cards = parse_cards(buf);

    println!("{}", solve1(&cards));
    println!("{}", solve2(&cards));
}
