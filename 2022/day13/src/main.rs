use std::cmp::Ordering;
use std::io::Read;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

fn parse_packet(s: &str) -> Packet {
    if s == "[]" {
        Packet::List(vec![])
    } else if !s.starts_with("[") {
        Packet::Integer(s.parse().unwrap())
    } else {
        let mut depth = 0;
        let mut prev = 1;
        let mut values = Vec::new();
        for i in 1..s.len() - 1 {
            match s.as_bytes()[i] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' if depth == 0 => {
                    values.push(parse_packet(&s[prev..i]));
                    prev = i + 1;
                }
                _ => {}
            }
        }
        values.push(parse_packet(&s[prev..s.len() - 1]));
        Packet::List(values)
    }
}

fn cmp_packet(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Integer(l), Packet::Integer(r)) => l.cmp(&r),
        (Packet::List(l), Packet::List(r)) => (0..l.len().min(r.len()))
            .map(|i| cmp_packet(&l[i], &r[i]))
            .skip_while(|ord| ord == &Ordering::Equal)
            .next()
            .unwrap_or(l.len().cmp(&r.len())),
        (Packet::Integer(_), Packet::List(_)) => {
            cmp_packet(&Packet::List(vec![left.clone()]), right)
        }
        (Packet::List(_), Packet::Integer(_)) => {
            cmp_packet(left, &Packet::List(vec![right.clone()]))
        }
    }
}

fn solve1(pairs: &Vec<Vec<Packet>>) -> usize {
    pairs
        .iter()
        .zip(1..)
        .filter(|(pair, _)| cmp_packet(&pair[0], &pair[1]) == Ordering::Less)
        .map(|(_, index)| index)
        .sum()
}

fn solve2(pairs: &Vec<Vec<Packet>>) -> usize {
    let mut packets = pairs.into_iter().flatten().cloned().collect::<Vec<_>>();
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_unstable_by(|a, b| cmp_packet(&a, &b));
    (packets.iter().position(|packet| packet == &div1).unwrap() + 1)
        * (packets.iter().position(|packet| packet == &div2).unwrap() + 1)
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let pairs = buf
        .split("\n\n")
        .map(|pair| {
            pair.split('\n')
                .map(|s| parse_packet(s))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&pairs));
    println!("{}", solve2(&pairs));
}
