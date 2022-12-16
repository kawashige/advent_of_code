use std::{collections::HashMap, io::Read};

fn recurse(
    current: usize,
    flows: &[usize],
    dist: &Vec<Vec<usize>>,
    opened: usize,
    minutes: usize,
    pressure: usize,
    pressures: &mut HashMap<usize, usize>,
) {
    for next in 0..flows.len() {
        if opened & 1 << next != 0 || minutes < dist[current][next] + 1 {
            continue;
        };
        let next_minutes = minutes - dist[current][next] - 1;
        let next_pressure = pressure + flows[next] * next_minutes;
        let next_opened = opened | 1 << next;

        if let Some(x) = pressures.get(&next_opened) {
            if x < &next_pressure {
                pressures.insert(next_opened, next_pressure);
            }
        } else {
            pressures.insert(next_opened, next_pressure);
        }

        recurse(
            next,
            flows,
            dist,
            next_opened,
            next_minutes,
            next_pressure,
            pressures,
        );
    }
}

fn solve1(flows: &[usize], dist: &Vec<Vec<usize>>) -> usize {
    let init = (0..flows.len()).fold(0, |acc, i| if flows[i] == 0 { acc | 1 << i } else { acc });
    let mut pressures = HashMap::new();

    recurse(0, flows, dist, init, 30, 0, &mut pressures);

    *pressures.values().max().unwrap()
}

fn solve2(flows: &[usize], dist: &Vec<Vec<usize>>) -> usize {
    let init = (0..flows.len()).fold(0, |acc, i| if flows[i] == 0 { acc | 1 << i } else { acc });
    let mut pressures = HashMap::new();

    recurse(0, flows, dist, init, 26, 0, &mut pressures);

    let mut max = 0;
    let keys = pressures.keys().collect::<Vec<_>>();
    for i in 0..keys.len() {
        for j in i..keys.len() {
            if keys[i] & keys[j] == init {
                max = max.max(pressures[keys[i]] + pressures[keys[j]]);
            }
        }
    }

    max
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let valves = buf
        .split('\n')
        .map(|line| {
            let sp = line.split(&[' ', '=', ';', ',']).collect::<Vec<_>>();
            (
                sp[1].to_string(),
                (
                    sp[5].parse::<usize>().unwrap(),
                    sp[11..]
                        .iter()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut labels = valves.keys().cloned().collect::<Vec<_>>();
    labels.sort_unstable();
    let label_index = labels.into_iter().zip(0..).collect::<HashMap<_, _>>();

    let mut flows = vec![0; label_index.len()];
    let mut list = vec![vec![]; label_index.len()];

    for (label, (flow, tunnels)) in valves {
        flows[label_index[&label]] = flow;
        list[label_index[&label]] = tunnels
            .into_iter()
            .map(|t| label_index[&t])
            .collect::<Vec<_>>();
    }

    let mut dist = vec![vec![std::usize::MAX; flows.len()]; flows.len()];
    for i in 0..dist.len() {
        dist[i][i] = 0;
        for t in &list[i] {
            dist[i][*t] = 1;
        }
    }

    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                if dist[i][k] != std::usize::MAX && dist[k][j] != std::usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    println!("{}", solve1(&flows, &dist));
    println!("{}", solve2(&flows, &dist));
}
