use std::io::Read;

fn parse_input(s: String) -> Vec<(Vec<i128>, Vec<i128>)> {
    s.split('\n')
        .map(|line| {
            let mut sp = line.split('@');
            let position = sp
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect::<Vec<_>>();
            let velocity = sp
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect::<Vec<_>>();
            (position, velocity)
        })
        .collect()
}

fn intersection(
    s1: &(Vec<i128>, Vec<i128>),
    s2: &(Vec<i128>, Vec<i128>),
) -> Option<(Vec<i128>, i128)> {
    let p1 = &s1.0;
    let p2 = &s2.0;
    let v1 = &s1.1;
    let v2 = &s2.1;

    if v2[0] * v1[1] == v1[0] * v2[1] {
        return None;
    }

    let x = (v1[0] * v2[0]).abs() * p2[1]
        - v1[0].abs() * v2[0].signum() * v2[1] * p2[0]
        - ((v1[0] * v2[0]).abs() * p1[1] - v2[0].abs() * v1[0].signum() * v1[1] * p1[0]);
    let y = v1[0].signum() * v1[1] * (v2[0].abs() * p2[1] - v2[0].signum() * v2[1] * p2[0])
        - (v2[0].signum() * v2[1] * (v1[0].abs() * p1[1] - v1[0].signum() * v1[1] * p1[0]));
    let multiplier = v2[0].abs() * v1[0].signum() * v1[1] - v1[0].abs() * v2[0].signum() * v2[1];

    Some((
        vec![x * multiplier.signum(), y * multiplier.signum()],
        multiplier.abs(),
    ))
}

fn solve1(hailstones: &Vec<(Vec<i128>, Vec<i128>)>) -> usize {
    let test_area = [200000000000000, 400000000000000];
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Some((intersection, multiplier)) = intersection(&hailstones[i], &hailstones[j]) {
                if (test_area[0] * multiplier..=test_area[1] * multiplier)
                    .contains(&intersection[0])
                    && (test_area[0] * multiplier..=test_area[1] * multiplier)
                        .contains(&intersection[1])
                    && ((hailstones[i].1[0] < 0
                        && intersection[0] <= hailstones[i].0[0] * multiplier)
                        || (hailstones[i].1[0] >= 0
                            && intersection[0] >= hailstones[i].0[0] * multiplier))
                    && ((hailstones[j].1[0] < 0
                        && intersection[0] <= hailstones[j].0[0] * multiplier)
                        || (hailstones[j].1[0] >= 0
                            && intersection[0] >= hailstones[j].0[0] * multiplier))
                    && ((hailstones[i].1[1] < 0
                        && intersection[1] <= hailstones[i].0[1] * multiplier)
                        || (hailstones[i].1[1] >= 0
                            && intersection[1] >= hailstones[i].0[1] * multiplier))
                    && ((hailstones[j].1[1] < 0
                        && intersection[1] <= hailstones[j].0[1] * multiplier)
                        || (hailstones[j].1[1] >= 0
                            && intersection[1] >= hailstones[j].0[1] * multiplier))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve2(hailstones: &Vec<(Vec<i128>, Vec<i128>)>) -> i128 {
    0
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let hailstones = parse_input(buf);

    println!("{:?}", hailstones);
    println!("{}", solve1(&hailstones));
    println!("{}", solve2(&hailstones));
}
