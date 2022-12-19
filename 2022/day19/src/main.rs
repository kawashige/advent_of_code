use std::{collections::HashMap, io::Read};

fn recurse(
    robots: [i32; 4],
    materials: [i32; 4],
    costs: &Vec<Vec<i32>>,
    max_robots: &[i32; 3],
    max_geodes: &mut i32,
    minutes: i32,
    memo: &mut HashMap<String, i32>,
) {
    if minutes == 0 {
        *max_geodes = std::cmp::max(*max_geodes, materials[3]);
        return;
    }
    if materials[3] + robots[3] * (minutes + 1) + (minutes * (minutes + 1)) / 2 <= *max_geodes {
        return;
    }

    let mut next_materials = materials.clone();
    for i in 0..robots.len() {
        next_materials[i] += robots[i];
    }
    for i in 0..robots.len() {
        if i < max_robots.len()
            && (robots[i] == max_robots[i] || max_robots[i] * (minutes - 1) <= materials[i])
        {
            continue;
        }
        if materials.iter().zip(costs[i].iter()).any(|(m, c)| m < c) {
            continue;
        }

        let mut next_materials2 = next_materials.clone();
        let mut next_robots2 = robots.clone();
        for m in 0..next_materials2.len() - 1 {
            next_materials2[m] -= costs[i][m];
        }
        next_robots2[i] += 1;
        recurse(
            next_robots2,
            next_materials2,
            costs,
            max_robots,
            max_geodes,
            minutes - 1,
            memo,
        );
    }
    recurse(
        robots,
        next_materials,
        costs,
        max_robots,
        max_geodes,
        minutes - 1,
        memo,
    );
}

fn solve1(costs: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut quality_level = 0;
    for i in 0..costs.len() {
        let mut robots = [0; 4];
        let materials = [0; 4];
        robots[0] = 1;
        let mut max_geodes = 0;
        let max_robots = costs[i].iter().fold([0; 3], |acc, cost| {
            [
                acc[0].max(cost[0]),
                acc[1].max(cost[1]),
                acc[2].max(cost[2]),
            ]
        });
        recurse(
            robots,
            materials,
            &costs[i],
            &max_robots,
            &mut max_geodes,
            24,
            &mut HashMap::new(),
        );

        quality_level += max_geodes * (i as i32 + 1);
    }
    quality_level
}

fn solve2(costs: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut result = 1;
    for i in 0..costs.len().min(3) {
        let mut robots = [0; 4];
        let materials = [0; 4];
        robots[0] = 1;
        let mut max_geodes = 0;
        let max_robots = costs[i].iter().fold([0; 3], |acc, cost| {
            [
                acc[0].max(cost[0]),
                acc[1].max(cost[1]),
                acc[2].max(cost[2]),
            ]
        });
        recurse(
            robots,
            materials,
            &costs[i],
            &max_robots,
            &mut max_geodes,
            32,
            &mut HashMap::new(),
        );
        result *= max_geodes;
    }
    result
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let costs = buf
        .split('\n')
        .map(|line| {
            let values = line
                .split_whitespace()
                .filter_map(|sp| sp.parse::<i32>().ok())
                .collect::<Vec<_>>();
            vec![
                vec![values[0], 0, 0],
                vec![values[1], 0, 0],
                vec![values[2], values[3], 0],
                vec![values[4], 0, values[5]],
            ]
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&costs));
    println!("{}", solve2(&costs));
}
