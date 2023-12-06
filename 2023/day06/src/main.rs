use std::io::Read;

fn parse_documents(s: String) -> (Vec<usize>, Vec<usize>) {
    let mut sp = s.split("\n");
    let time = sp
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let distance = sp
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect::<Vec<_>>();
    (time, distance)
}

fn solve1(time: &[usize], distance: &[usize]) -> usize {
    (0..time.len())
        .map(|i| {
            (1..time[i])
                .filter(|t| distance[i] < t * (time[i] - t))
                .count()
        })
        .product()
}

fn solve2(time: &[usize], distance: &[usize]) -> usize {
    let time = time
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = distance
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    (1..time).filter(|t| distance < t * (time - t)).count()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (time, distance) = parse_documents(buf);

    println!("{}", solve1(&time, &distance));
    println!("{}", solve2(&time, &distance));
}
