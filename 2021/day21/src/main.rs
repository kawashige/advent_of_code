use proconio::input;

fn solve(s1: usize, s2: usize) -> usize {
    let mut p1 = s1;
    let mut p2 = s2;
    let mut score1 = 0;
    let mut score2 = 0;

    let mut i = 1;
    let mut count = 0;
    while score1 < 1000 && score2 < 1000 {
        p1 = (p1 + 3 * (i + 1)) % 10;
        if p1 == 0 {
            p1 = 10;
        }
        score1 += p1;
        i = (i + 3) % 100;
        if i == 0 {
            i = 100;
        }
        count += 3;

        if 1000 <= score1 {
            break;
        }

        p2 = (p2 + 3 * (i + 1)) % 10;
        if p2 == 0 {
            p2 = 10;
        }
        score2 += p2;
        i = (i + 3) % 100;
        if i == 0 {
            i = 100;
        }
        count += 3;
    }

    count * if 1000 <= score1 { score2 } else { score1 }
}

fn solve2(s1: usize, s2: usize) -> usize {
    let dice = (1..4)
        .flat_map(|i| {
            (1..4)
                .flat_map(|j| (1..4).map(|k| i + j + k).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut win1 = 0;
    let mut win2 = 0;

    let mut dp = vec![vec![vec![vec![0; 11]; 11]; 21]; 21];
    dp[0][0][s1][s2] = 1;

    let mut finished = false;
    let mut turn = 0;
    while !finished {
        finished = true;
        let mut new_dp = vec![vec![vec![vec![0; 11]; 11]; 21]; 21];

        for s1 in 0..21 {
            for s2 in 0..21 {
                for p1 in 1..11 {
                    for p2 in 1..11 {
                        if dp[s1][s2][p1][p2] == 0 {
                            continue;
                        }
                        for d in dice.iter() {
                            if turn == 0 {
                                let mut next = (p1 + d) % 10;
                                if next == 0 {
                                    next = 10;
                                }
                                if 21 <= s1 + next {
                                    win1 += dp[s1][s2][p1][p2];
                                } else {
                                    new_dp[s1 + next][s2][next][p2] += dp[s1][s2][p1][p2];
                                    finished = false;
                                }
                            } else {
                                let mut next = (p2 + d) % 10;
                                if next == 0 {
                                    next = 10;
                                }
                                if 21 <= s2 + next {
                                    win2 += dp[s1][s2][p1][p2];
                                } else {
                                    new_dp[s1][s2 + next][p1][next] += dp[s1][s2][p1][p2];
                                    finished = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        dp = new_dp;
        turn = (turn + 1) % 2;
    }

    win1.max(win2)
}

fn main() {
    input! {
        s1: usize,
        s2: usize
    }

    println!("part1: {}", solve(s1, s2));
    println!("part2: {}", solve2(s1, s2));
}
