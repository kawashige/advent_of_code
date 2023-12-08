use std::{collections::HashMap, io::Read};

fn parse_document(s: String) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut sp = s.split("\n\n");
    let instructions = sp.next().unwrap().chars().collect::<Vec<_>>();
    let network = sp
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let key = line[..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (key, (left, right))
        })
        .collect::<HashMap<_, _>>();
    (instructions, network)
}

fn solve1(instructions: &[char], network: &HashMap<String, (String, String)>) -> usize {
    let mut current = "AAA";

    for i in 0.. {
        if current == "ZZZ" {
            return i;
        }
        current = if instructions[i % instructions.len()] == 'L' {
            &network[current].0
        } else {
            &network[current].1
        }
    }

    0
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn solve2(instructions: &[char], network: &HashMap<String, (String, String)>) -> usize {
    let nodes = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();
    let mut indices = Vec::new();

    for node in nodes {
        let mut current = node;

        for i in 0.. {
            if current.ends_with("Z") {
                indices.push(i);
                break;
            }

            current = if instructions[i % instructions.len()] == 'L' {
                &network[current].0
            } else {
                &network[current].1
            }
        }
    }
    indices.into_iter().fold(1, |acc, i| lcm(acc, i))
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (instructions, network) = parse_document(buf);

    println!("{}", solve1(&instructions, &network));
    println!("{}", solve2(&instructions, &network));
}
