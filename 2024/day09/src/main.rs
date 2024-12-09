use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let map = buf
        .as_bytes()
        .iter()
        .map(|b| (b - b'0') as usize)
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn solve1(map: &[usize]) -> usize {
    let mut rearranged = Vec::new();
    for i in 0..map.len() {
        for _ in 0..map[i] {
            rearranged.push(if i % 2 == 0 { i / 2 } else { std::usize::MAX });
        }
    }
    let mut prev = 0;
    for i in (0..rearranged.len()).rev() {
        if rearranged[i] == std::usize::MAX {
            continue;
        }
        if let Some(j) = (prev..i).find(|j| rearranged[*j] == std::usize::MAX) {
            rearranged[j] = rearranged[i];
            rearranged[i] = std::usize::MAX;
            prev = j + 1;
        } else {
            break;
        }
    }

    let mut result = 0;
    for i in 0..rearranged.len() {
        if rearranged[i] == std::usize::MAX {
            break;
        }
        result += i * rearranged[i];
    }
    result
}

fn solve2(map: &[usize]) -> usize {
    let mut rearranged = Vec::new();
    for i in 0..map.len() {
        if map[i] == 0 {
            continue;
        }
        rearranged.push((i % 2, map[i], i / 2));
    }

    let mut i = rearranged.len() - 1;
    while 0 < i {
        if rearranged[i].0 == 1 {
            i -= 1;
            continue;
        }
        for j in 0..i {
            if rearranged[j].0 == 0 || rearranged[j].1 < rearranged[i].1 {
                continue;
            }
            let d = rearranged[j].1 - rearranged[i].1;
            rearranged[j] = rearranged[i];
            rearranged[i].0 = 1;
            if 0 < d {
                if j + 1 < rearranged.len() && rearranged[j + 1].0 == 1 {
                    rearranged[j + 1].1 += d;
                } else {
                    rearranged.insert(j + 1, (1, d, 0));
                    i += 1;
                }
            }
            break;
        }
        i -= 1;
    }

    let mut result = 0;
    let mut j = 0;
    for i in 0..rearranged.len() {
        if rearranged[i].0 == 0 {
            result += rearranged[i].2 * ((j + j + rearranged[i].1 - 1) * rearranged[i].1) / 2;
        }
        j += rearranged[i].1;
    }
    result
}
