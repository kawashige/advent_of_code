use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (rules, updates) = {
        let mut sp = buf.split("\n\n");
        let rules = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|l| {
                l.split('|')
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let updates = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|l| {
                l.split(',')
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (rules, updates)
    };

    println!("{:?}", solve1(&rules, &updates));
    println!("{:?}", solve2(&rules, &updates));
}

fn solve1(rules: &Vec<Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut before = vec![vec![false; 100]; 100];
    for i in 0..rules.len() {
        before[rules[i][1]][rules[i][0]] = true;
    }

    updates
        .iter()
        .filter(|update| {
            let mut is_right = true;
            for i in 0..update.len() {
                for j in 0..i {
                    if before[update[j]][update[i]] {
                        is_right = false;
                        break;
                    }
                }
                if !is_right {
                    break;
                }
            }
            is_right
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve2(rules: &Vec<Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut before = vec![vec![false; 100]; 100];
    for i in 0..rules.len() {
        before[rules[i][1]][rules[i][0]] = true;
    }

    updates
        .iter()
        .map(|update| {
            let mut count = vec![0; update.len()];
            for i in 0..update.len() {
                for j in 0..update.len() {
                    if i != j && before[update[i]][update[j]] {
                        count[i] += 1;
                    }
                }
            }
            if (0..update.len()).all(|i| count[i] <= i) {
                0
            } else {
                let mut update = count
                    .into_iter()
                    .zip(update.clone().into_iter())
                    .collect::<Vec<_>>();
                update.sort_unstable();
                update[update.len() / 2].1
            }
        })
        .sum()
}
