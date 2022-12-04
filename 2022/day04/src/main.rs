use proconio::input;

fn solve1(pairs: &[((usize, usize), (usize, usize))]) -> usize {
    pairs
        .into_iter()
        .filter(|((s1, e1), (s2, e2))| (s1 <= s2 && e2 <= e1) || (s2 <= s1 && e1 <= e2))
        .count()
}

fn solve2(pairs: &[((usize, usize), (usize, usize))]) -> usize {
    pairs
        .into_iter()
        .filter(|((s1, e1), (s2, e2))| !(e1 < s2) && !(e2 < s1))
        .count()
}
fn main() {
    input! {
        n: usize,
        pairs: [String; n]
    }

    let pairs = pairs
        .into_iter()
        .map(|pair| {
            let mut sp = pair.split(",");
            let mut first = sp.next().unwrap().split("-");
            let mut second = sp.next().unwrap().split("-");
            (
                (
                    first.next().unwrap().parse::<usize>().unwrap(),
                    first.next().unwrap().parse::<usize>().unwrap(),
                ),
                (
                    second.next().unwrap().parse::<usize>().unwrap(),
                    second.next().unwrap().parse::<usize>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&pairs));
    println!("{}", solve2(&pairs));
}
