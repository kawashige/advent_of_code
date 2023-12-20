use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

#[derive(Debug, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjection,
}
fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
fn parse_input(s: String) -> HashMap<String, (Module, Vec<String>)> {
    s.split('\n')
        .map(|line| {
            let mut sp = line.split(" -> ");
            let name = sp.next().unwrap();
            let module = if name.starts_with('%') {
                Module::FlipFlop
            } else if name.starts_with('&') {
                Module::Conjection
            } else {
                Module::Broadcaster
            };
            let destination = sp
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            (
                name.trim_start_matches('&')
                    .trim_start_matches('%')
                    .to_string(),
                (module, destination),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn solve1(modules: &HashMap<String, (Module, Vec<String>)>) -> usize {
    let mut flip_flop = HashMap::new();
    let mut conjection = HashMap::new();
    for (name, (module, destinations)) in modules {
        if module == &Module::FlipFlop {
            flip_flop.insert(name.to_string(), 0);
        }
        for destination in destinations {
            if modules.contains_key(destination) && modules[destination].0 == Module::Conjection {
                (*conjection
                    .entry(destination.to_string())
                    .or_insert(HashMap::new()))
                .insert(name.to_string(), 0);
            }
        }
    }

    let mut pulse_count = [0; 2];
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back((0, "broadcaster", "boardcaster"));

        while let Some((pulse, destination, from)) = pulses.pop_front() {
            pulse_count[pulse] += 1;
            if let Some((module, destinations)) = modules.get(destination) {
                match module {
                    Module::Broadcaster => {
                        for next in destinations {
                            pulses.push_back((pulse, next, destination));
                        }
                    }
                    Module::FlipFlop if pulse == 0 => {
                        let next_pulse = (flip_flop[destination] + 1) % 2;
                        flip_flop.insert(destination.to_string(), next_pulse);
                        for next in destinations {
                            pulses.push_back((next_pulse, next, destination));
                        }
                    }
                    Module::Conjection => {
                        *conjection
                            .get_mut(destination)
                            .unwrap()
                            .get_mut(from)
                            .unwrap() = pulse;
                        if conjection[destination].values().all(|v| v == &1) {
                            for next in destinations {
                                pulses.push_back((0, next, destination));
                            }
                        } else {
                            for next in destinations {
                                pulses.push_back((1, next, destination));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    pulse_count[0] * pulse_count[1]
}

fn solve2(modules: &HashMap<String, (Module, Vec<String>)>) -> usize {
    let mut acc = 1;

    for start in &modules["broadcaster"].1 {
        let mut keys = vec![];
        let mut end_keys = vec![];

        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            keys.push(node.clone());
            for destination in &modules[node].1 {
                if modules[destination].0 == Module::Conjection {
                    end_keys.push(node);
                } else {
                    stack.push(destination);
                }
            }
        }

        let destinations = keys
            .iter()
            .map(|key| {
                modules[key]
                    .1
                    .iter()
                    .filter_map(|k| (0..keys.len()).find(|i| &keys[*i] == k))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut state = vec![false; keys.len()];
        let end_indices = end_keys
            .into_iter()
            .map(|key| (0..keys.len()).find(|i| &keys[*i] == key).unwrap())
            .collect::<Vec<_>>();
        let start_index = (0..keys.len()).find(|i| &keys[*i] == start).unwrap();

        for i in 1.. {
            let mut pulses = VecDeque::new();
            pulses.push_back(start_index);

            while let Some(destination) = pulses.pop_front() {
                state[destination] = !state[destination];
                if !state[destination] {
                    for next in &destinations[destination] {
                        pulses.push_back(*next);
                    }
                }
            }

            if end_indices.iter().all(|j| state[*j]) {
                acc = lcm(acc, i);
                break;
            }
        }
    }

    acc
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let modules = parse_input(buf);

    println!("{}", solve1(&modules));
    println!("{}", solve2(&modules));
}
