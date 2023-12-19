use std::{collections::HashMap, io::Read};

pub fn parse_input(
    s: String,
) -> (
    HashMap<String, (Vec<(usize, char, usize, String)>, String)>,
    Vec<Vec<usize>>,
) {
    let mut sp = s.split("\n\n");
    let workflows = sp
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let mut sp = line.trim_end_matches("}").split("{");
            let label = sp.next().unwrap().to_string();
            let mut rules = sp.next().unwrap().split(",").collect::<Vec<_>>();
            let default = rules.pop().unwrap().to_string();
            let rules = rules
                .into_iter()
                .map(|rule| {
                    let mut sp = rule.split(':');
                    let condidion = sp.next().unwrap();
                    let next_label = sp.next().unwrap().to_string();
                    (
                        match condidion.as_bytes()[0] {
                            b'x' => 0,
                            b'm' => 1,
                            b'a' => 2,
                            b's' => 3,
                            _ => unreachable!(),
                        },
                        condidion.as_bytes()[1] as char,
                        condidion[2..].parse::<usize>().unwrap(),
                        next_label,
                    )
                })
                .collect::<Vec<_>>();
            (label, (rules, default))
        })
        .collect::<HashMap<_, _>>();
    let ratings = sp
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split(",")
                .map(|part| part[2..].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (workflows, ratings)
}

fn solve1(
    workflows: &HashMap<String, (Vec<(usize, char, usize, String)>, String)>,
    ratings: &Vec<Vec<usize>>,
) -> usize {
    ratings
        .iter()
        .filter(|rating| {
            let mut current = "in";
            while current != "A" && current != "R" {
                if let Some((rules, default)) = workflows.get(current) {
                    let mut found = false;
                    for (i, op, val, next) in rules {
                        let matched = if op == &'<' {
                            &rating[*i] < val
                        } else {
                            &rating[*i] > val
                        };
                        if matched {
                            current = next;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        current = default;
                    }
                }
            }

            current == "A"
        })
        .map(|rating| rating.iter().sum::<usize>())
        .sum()
}

fn solve2(workflows: &HashMap<String, (Vec<(usize, char, usize, String)>, String)>) -> usize {
    let mut count = 0;
    let mut stack = vec![("in", vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)])];

    while let Some((current, ranges)) = stack.pop() {
        if current == "A" {
            count += ranges
                .iter()
                .map(|(min, max)| max - min + 1)
                .product::<usize>();
            continue;
        }
        if current == "R" {
            continue;
        }

        if let Some((rules, default)) = workflows.get(current) {
            let mut found = false;
            for (i, op, val, next) in rules {
                if op == &'<' {
                    if &ranges[*i].0 < val {
                        let mut new_ranges = ranges.clone();
                        new_ranges[*i] = (ranges[*i].0, (val - 1).min(ranges[*i].1));
                        stack.push((next, new_ranges));
                        if val <= &ranges[*i].1 {
                            let mut new_ranges2 = ranges.clone();
                            new_ranges2[*i] = (*val, ranges[*i].1);
                            stack.push((current, new_ranges2));
                        }
                        found = true;
                        break;
                    }
                } else {
                    if val < &ranges[*i].1 {
                        let mut new_ranges = ranges.clone();
                        new_ranges[*i] = ((val + 1).max(ranges[*i].0), ranges[*i].1);
                        stack.push((next, new_ranges));
                        if &ranges[*i].0 <= val {
                            let mut new_ranges2 = ranges.clone();
                            new_ranges2[*i] = (ranges[*i].0, *val);
                            stack.push((current, new_ranges2));
                        }
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                stack.push((default, ranges));
            }
        }
    }

    count
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (workflows, ratings) = parse_input(buf);

    println!("{}", solve1(&workflows, &ratings));
    println!("{}", solve2(&workflows));
}
