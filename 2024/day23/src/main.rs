use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let list = buf
        .split("\n")
        .map(|l| l.split("-").map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&list));
    println!("{:?}", solve2(&list));
}

fn solve1(list: &Vec<Vec<String>>) -> usize {
    let computers = list
        .iter()
        .cloned()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let indices = computers
        .iter()
        .cloned()
        .zip(0..)
        .collect::<HashMap<_, _>>();

    let mut matrix = vec![vec![false; computers.len()]; computers.len()];
    for connection in list {
        matrix[indices[&connection[0]]][indices[&connection[1]]] = true;
        matrix[indices[&connection[1]]][indices[&connection[0]]] = true;
    }

    let mut count = 0;
    for i in 0..computers.len() {
        for j in 0..i {
            if !matrix[i][j] {
                continue;
            }
            for k in 0..j {
                if !matrix[i][k] || !matrix[j][k] {
                    continue;
                }
                if computers[i].starts_with("t")
                    || computers[j].starts_with("t")
                    || computers[k].starts_with("t")
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn dfs(i: usize, matrix: &Vec<Vec<bool>>, group: &mut Vec<usize>, max_group: &mut Vec<usize>) {
    if i == matrix.len() {
        if group.len() >= max_group.len() {
            *max_group = group.clone();
        }
        return;
    }
    if group.is_empty() || (0..group.len()).all(|j| matrix[i][group[j]]) {
        group.push(i);
        dfs(i + 1, matrix, group, max_group);
        group.pop();
    }
    dfs(i + 1, matrix, group, max_group);
}

fn solve2(list: &Vec<Vec<String>>) -> String {
    let mut computers = list
        .iter()
        .cloned()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    computers.sort_unstable();
    let indices = computers
        .iter()
        .cloned()
        .zip(0..)
        .collect::<HashMap<_, _>>();

    let mut matrix = vec![vec![false; computers.len()]; computers.len()];
    for connection in list {
        matrix[indices[&connection[0]]][indices[&connection[1]]] = true;
        matrix[indices[&connection[1]]][indices[&connection[0]]] = true;
    }

    let mut max_group = Vec::new();
    dfs(0, &matrix, &mut Vec::new(), &mut max_group);
    let mut max_group = max_group
        .into_iter()
        .map(|i| computers[i].clone())
        .collect::<Vec<_>>();
    max_group.sort_unstable();
    max_group.join(",")
}
