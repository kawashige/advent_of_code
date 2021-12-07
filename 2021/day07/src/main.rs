use proconio::input;

fn solve(positions: &[i32]) -> usize {
    let medium = positions[positions.len() / 2];
    positions.iter().map(|p| (p - medium).abs() as usize).sum()
}

fn solve2(positions: &[i32]) -> usize {
    let mut cost = vec![0; (positions[positions.len() - 1] - positions[0]) as usize + 1];
    for i in 1..cost.len() {
        cost[i] = cost[i - 1] + i;
    }

    (positions[0]..=*positions.last().unwrap())
        .map(|p1| {
            positions
                .iter()
                .map(|p2| cost[(p2 - p1).abs() as usize])
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

fn main() {
    input! {
        positions: String
    }

    let mut positions = positions
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    positions.sort_unstable();

    println!("part1: {}", solve(&positions));
    println!("part2: {}", solve2(&positions));
}
