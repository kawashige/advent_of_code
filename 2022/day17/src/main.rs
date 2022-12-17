use std::{collections::HashMap, io::Read};

fn move_jet(line: usize, jet: char) -> usize {
    if jet == '<' {
        line << 1
    } else {
        line >> 1
    }
}

fn solve1(jets: &[char], rocks: &Vec<Vec<usize>>) -> usize {
    let mut i = 0;
    let mut chamber = std::iter::once(0b111111111)
        .chain(std::iter::repeat(0b100000001).take(2022 * 4))
        .collect::<Vec<_>>();

    for j in 0..2022 {
        let mut rock = rocks[j % rocks.len()].clone();
        let mut depth = (0..chamber.len())
            .find(|k| chamber[*k] == 0b100000001)
            .unwrap()
            + 2
            + rock.len();

        loop {
            let jet = jets[i % jets.len()];
            if (0..rock.len()).all(|k| move_jet(rock[k], jet) & chamber[depth - k] == 0) {
                rock = rock.into_iter().map(|line| move_jet(line, jet)).collect();
            }
            i += 1;

            if (0..rock.len()).all(|k| rock[k] & chamber[depth - 1 - k] == 0) {
                depth -= 1;
            } else {
                for k in 0..rock.len() {
                    chamber[depth - k] |= rock[k];
                }
                break;
            }
        }
    }

    (0..chamber.len())
        .rev()
        .find(|k| chamber[*k] != 0b100000001)
        .unwrap()
}

fn solve2(jets: &[char], rocks: &Vec<Vec<usize>>) -> usize {
    const ROCK_COUNT: usize = 1_000_000_000_000;

    let mut i = 0;
    let mut chamber = vec![0b111111111];

    let mut state = HashMap::new();
    let mut bottom = 0;

    let mut j = 0;
    while j < ROCK_COUNT {
        let depths = (1..8)
            .map(|k| {
                (0..chamber.len())
                    .rev()
                    .find(|l| chamber[*l] & 1 << k != 0)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let min = *depths.iter().min().unwrap();
        let key = format!(
            "{},{},{}",
            i % jets.len(),
            j % rocks.len(),
            depths
                .into_iter()
                .map(|d| (d - min).to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        if let Some((b, index)) = state.get(&key) {
            let count = (ROCK_COUNT - j) / (j - index);
            bottom += count * (min - b);
            j += (j - index) * count;
        } else {
            state.insert(key, (min, j));
        }

        let mut rock = rocks[j % rocks.len()].clone();
        for _ in 0..3 + rock.len() {
            chamber.push(0b100000001);
        }
        let mut depth = (0..chamber.len())
            .find(|k| chamber[*k] == 0b100000001)
            .unwrap()
            + 2
            + rock.len();

        loop {
            let jet = jets[i % jets.len()];
            if (0..rock.len()).all(|k| move_jet(rock[k], jet) & chamber[depth - k] == 0) {
                rock = rock.into_iter().map(|line| move_jet(line, jet)).collect();
            }
            i += 1;

            if (0..rock.len()).all(|k| rock[k] & chamber[depth - 1 - k] == 0) {
                depth -= 1;
            } else {
                for k in 0..rock.len() {
                    chamber[depth - k] |= rock[k];
                }
                break;
            }
        }
        j += 1;
    }

    (0..chamber.len())
        .rev()
        .find(|k| chamber[*k] != 0b100000001)
        .unwrap()
        + bottom
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let jets = buf.chars().collect::<Vec<_>>();
    let rocks = vec![
        vec![0b00111100],
        vec![0b00010000, 0b00111000, 0b00010000],
        vec![0b00001000, 0b00001000, 0b00111000],
        vec![0b00100000, 0b00100000, 0b00100000, 0b00100000],
        vec![0b00110000, 0b00110000],
    ];

    println!("{}", solve1(&jets, &rocks));
    println!("{}", solve2(&jets, &rocks));
}
