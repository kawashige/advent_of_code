use proconio::input;

fn solve(commands: &[(String, usize)]) -> (usize, usize) {
    commands
        .iter()
        .fold((0, 0), |(h, d), (command, unit)| match command.as_str() {
            "forward" => (h + unit, d),
            "down" => (h, d + unit),
            "up" => (h, d - unit),
            _ => unreachable!(),
        })
}

fn solve2(commands: &[(String, usize)]) -> (usize, usize, usize) {
    commands
        .iter()
        .fold((0, 0, 0), |(h, d, a), (command, unit)| {
            match command.as_str() {
                "forward" => (h + unit, d + a * unit, a),
                "down" => (h, d, a + unit),
                "up" => (h, d, a - unit),
                _ => unreachable!(),
            }
        })
}

fn main() {
    input! {
        n: usize,
        commands: [(String, usize); n]
    }

    let (h, d) = solve(&commands);
    println!("part1: {}", h * d);

    let (h, d, _) = solve2(&commands);
    println!("part2: {}", h * d);
}
