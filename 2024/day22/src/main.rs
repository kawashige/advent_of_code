use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let secret_numbers = buf
        .split("\n")
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&secret_numbers));
    println!("{:?}", solve2(&secret_numbers));
}

fn next(s: usize) -> usize {
    const M: usize = 16777216;
    let mut s = s;
    s = (s ^ (s * 64)) % M;
    s = (s ^ (s / 32)) % M;
    s = (s ^ (s * 2048)) % M;
    s
}

fn solve1(secret_numbers: &Vec<usize>) -> usize {
    secret_numbers
        .iter()
        .map(|s| {
            let mut s = *s;
            for _ in 0..2000 {
                s = next(s);
            }
            s
        })
        .sum()
}

fn solve2(secret_numbers: &Vec<usize>) -> usize {
    let mut bananas = vec![vec![vec![vec![0; 19]; 19]; 19]; 19];
    let mut max = 0;
    for secret_number in secret_numbers {
        let mut s = *secret_number;
        let mut queue = VecDeque::new();
        let mut prev = (s % 10) as i32;
        let mut set = HashSet::new();
        for _ in 0..2000 {
            s = next(s);
            let d = (s % 10) as i32;
            queue.push_back(prev - d);
            prev = d;
            if 4 < queue.len() {
                queue.pop_front();
            }
            if queue.len() == 4 {
                if set.insert((queue[0], queue[1], queue[2], queue[3])) {
                    bananas[(queue[0] + 9) as usize][(queue[1] + 9) as usize]
                        [(queue[2] + 9) as usize][(queue[3] + 9) as usize] += d as usize;
                    max = max.max(
                        bananas[(queue[0] + 9) as usize][(queue[1] + 9) as usize]
                            [(queue[2] + 9) as usize][(queue[3] + 9) as usize],
                    );
                }
            }
        }
    }
    max
}
