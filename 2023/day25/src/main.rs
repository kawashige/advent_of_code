use std::{
    collections::{HashMap, HashSet},
    io::Read,
};
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let parent = self.root(self.parent[i]);
        self.parent[i] = parent;
        parent
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let mut parent_i = self.root(i);
        let mut parent_j = self.root(j);

        if parent_i == parent_j {
            return;
        }

        if self.size(parent_i) < self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

fn parse_input(s: String) -> Vec<Vec<usize>> {
    let components = s
        .split('\n')
        .map(|line| {
            let mut sp = line.split(":");
            let label = sp.next().unwrap().to_string();
            let connected = sp
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            (label, connected)
        })
        .collect::<Vec<_>>();

    let labels = components
        .iter()
        .flat_map(|(label, connected)| {
            connected
                .clone()
                .into_iter()
                .chain(std::iter::once(label.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .zip(0..)
        .collect::<HashMap<_, _>>();

    let mut connected = vec![vec![]; labels.len()];

    for (label, next) in components {
        for next_component in next {
            connected[labels[&label]].push(labels[&next_component]);
        }
    }

    connected
}

fn solve1(connected: &Vec<Vec<usize>>) -> usize {
    for i in 0..connected.len() {
        for j in i + 1..connected.len() {
            let mut disjoint_set = DisjointSet::new(connected.len());
            let mut disconnected = 0;

            for k in 0..connected.len() {
                for l in 0..connected[k].len() {
                    if (disjoint_set.root(i) == disjoint_set.root(k)
                        && disjoint_set.root(j) == disjoint_set.root(connected[k][l]))
                        || (disjoint_set.root(j) == disjoint_set.root(k)
                            && disjoint_set.root(i) == disjoint_set.root(connected[k][l]))
                    {
                        disconnected += 1;
                        continue;
                    }
                    disjoint_set.unite(k, connected[k][l]);
                }
            }

            if disconnected == 3 {
                let root = [disjoint_set.root(i), disjoint_set.root(j)];
                return disjoint_set.size(root[0]) * disjoint_set.size(root[1]);
            }
        }
    }

    0
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let connected = parse_input(buf);
    println!("{}", solve1(&connected));
}
