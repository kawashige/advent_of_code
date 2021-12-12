use std::collections::{HashMap, HashSet};

use proconio::input;

fn solve(list: &HashMap<String, Vec<String>>) -> usize {
    let mut count = 0;
    let mut seen = vec!["start".to_string()].into_iter().collect();
    dfs(list, "start".to_string(), &mut seen, &mut count, true);
    count
}

fn solve2(list: &HashMap<String, Vec<String>>) -> usize {
    let mut count = 0;
    let mut seen = vec!["start".to_string()].into_iter().collect();
    dfs(list, "start".to_string(), &mut seen, &mut count, false);
    count
}

fn dfs(
    list: &HashMap<String, Vec<String>>,
    current: String,
    seen: &mut HashSet<String>,
    count: &mut usize,
    use_small_cave_twice: bool,
) {
    if &current == "end" {
        *count += 1;
        return;
    }

    if let Some(nodes) = list.get(&current) {
        for next in nodes {
            let is_small_cave = next.chars().next().unwrap().is_ascii_lowercase();
            let mut is_twice = false;
            if is_small_cave && seen.contains(next) {
                if next == "start" || use_small_cave_twice {
                    continue;
                } else {
                    is_twice = true;
                }
            }
            if is_small_cave {
                seen.insert(next.to_string());
            }
            dfs(
                list,
                next.to_string(),
                seen,
                count,
                is_twice || use_small_cave_twice,
            );
            if is_small_cave && !is_twice {
                seen.remove(next);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        edges: [String; n]
    }

    let mut list = HashMap::new();
    for edge in edges {
        let from_to = edge.split("-").collect::<Vec<_>>();
        (*list.entry(from_to[0].to_string()).or_insert(Vec::new())).push(from_to[1].to_string());
        (*list.entry(from_to[1].to_string()).or_insert(Vec::new())).push(from_to[0].to_string());
    }

    println!("part1: {}", solve(&list));
    println!("part2: {}", solve2(&list));
}
