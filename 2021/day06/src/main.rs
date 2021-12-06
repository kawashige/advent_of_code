use proconio::input;

fn solve(state: &[usize]) -> usize {
    simulate(state, 80)
}

fn solve2(state: &[usize]) -> usize {
    simulate(state, 256)
}

fn simulate(state: &[usize], days: usize) -> usize {
    let mut remains_count = state.iter().fold([0; 9], |mut count, s| {
        count[*s] += 1;
        count
    });

    for _ in 0..days {
        remains_count.rotate_left(1);
        remains_count[6] += remains_count[8];
    }

    remains_count.iter().sum()
}

fn main() {
    input! {
        state: String
    }

    let state = state
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", solve(&state));
    println!("{}", solve2(&state));
}
