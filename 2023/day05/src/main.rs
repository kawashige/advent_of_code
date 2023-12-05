use std::io::Read;

fn parse_almanac(s: String) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let sp = s.split("\n\n").collect::<Vec<_>>();
    let seeds = sp[0]
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let maps = sp[1..]
        .iter()
        .map(|map| {
            map.split("\n")
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn convert(seed: &usize, maps: &[Vec<Vec<usize>>]) -> usize {
    let mut current = *seed;
    for map in maps {
        for line in map {
            if (line[1]..line[1] + line[2]).contains(&current) {
                current = line[0] + current - line[1];
                break;
            }
        }
    }
    current
}

fn convert2(seeds: Vec<Vec<usize>>, maps: &[Vec<Vec<usize>>]) -> Vec<Vec<usize>> {
    let mut current = seeds;
    current.sort_unstable();

    for map in maps {
        let mut converted = Vec::new();

        while let Some(seed) = current.pop() {
            let mut not_converted = true;
            for line in map {
                let src = vec![line[1], line[1] + line[2] - 1];
                if src[0] <= seed[0] && seed[1] <= src[1] {
                    converted.push(vec![
                        line[0] + seed[0] - line[1],
                        line[0] + seed[1] - line[1],
                    ]);
                    not_converted = false;
                    break;
                } else if src[0] <= seed[0] && (seed[0]..=seed[1]).contains(&src[1]) {
                    converted.push(vec![
                        line[0] + seed[0] - line[1],
                        line[0] + src[1] - line[1],
                    ]);
                    current.push(vec![src[1] + 1, seed[1]]);
                    not_converted = false;
                    break;
                } else if (seed[0]..=seed[1]).contains(&src[0]) && seed[1] < src[1] {
                    converted.push(vec![
                        line[0] + src[0] - line[1],
                        line[0] + seed[1] - line[1],
                    ]);
                    current.push(vec![seed[0], src[0] - 1]);
                    not_converted = false;
                    break;
                } else if (seed[0]..=seed[1]).contains(&src[0])
                    && (seed[0]..=seed[1]).contains(&src[1])
                {
                    converted.push(vec![line[0] + src[0] - line[1], line[0] + src[1] - line[1]]);
                    current.push(vec![seed[0], src[0] - 1]);
                    current.push(vec![src[1] + 1, seed[1]]);
                    not_converted = false;
                    break;
                }
            }
            if not_converted {
                converted.push(seed);
            }
        }

        current = converted;
    }

    current
}

fn solve1(seeds: &[usize], maps: &[Vec<Vec<usize>>]) -> usize {
    seeds.iter().map(|seed| convert(seed, maps)).min().unwrap()
}

fn solve2(seeds: &[usize], maps: &[Vec<Vec<usize>>]) -> usize {
    let seeds = seeds
        .chunks(2)
        .map(|chunk| vec![chunk[0], chunk[0] + chunk[1] - 1])
        .collect::<Vec<_>>();

    let mut converted = convert2(seeds, maps);
    converted.sort_unstable();
    converted[0][0]
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (seeds, maps) = parse_almanac(buf);

    println!("{}", solve1(&seeds, &maps));
    println!("{}", solve2(&seeds, &maps));
}
