use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        map: [Chars; n]
    }

    println!("{:?}", solve1(&map));
    println!("{:?}", solve2(&map));
}

fn recurse(
    i: usize,
    j: usize,
    map: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    area: &mut usize,
    perimeter: &mut usize,
) {
    if seen[i][j] {
        return;
    }
    seen[i][j] = true;

    let mut p = 4;
    for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
        if !(0..map.len() as i32).contains(&new_i)
            || !(0..map[0].len() as i32).contains(&new_j)
            || map[i][j] != map[new_i as usize][new_j as usize]
        {
            continue;
        }
        p -= 1;
        recurse(new_i as usize, new_j as usize, map, seen, area, perimeter);
    }

    *area += 1;
    *perimeter += p;
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if seen[i][j] {
                continue;
            }
            let mut area = 0;
            let mut perimeter = 0;
            recurse(i, j, &map, &mut seen, &mut area, &mut perimeter);
            result += area * perimeter;
        }
    }

    result
}

fn recurse2(
    i: usize,
    j: usize,
    g: usize,
    map: &Vec<Vec<char>>,
    group: &mut Vec<Vec<usize>>,
    area: &mut usize,
) {
    if group[i][j] != std::usize::MAX {
        return;
    }
    group[i][j] = g;

    let d = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for k in 0..d.len() {
        let (new_i, new_j) = (i as i32 + d[k].0, j as i32 + d[k].1);
        if !(0..map.len() as i32).contains(&new_i)
            || !(0..map[0].len() as i32).contains(&new_j)
            || map[i][j] != map[new_i as usize][new_j as usize]
        {
            continue;
        }
        recurse2(new_i as usize, new_j as usize, g, map, group, area);
    }
    *area += 1;
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut group = vec![vec![std::usize::MAX; map[0].len()]; map.len()];
    let mut areas = Vec::new();
    let mut g = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if group[i][j] != std::usize::MAX {
                continue;
            }
            let mut area = 0;
            recurse2(i, j, g, &map, &mut group, &mut area);
            areas.push(area);
            g += 1;
        }
    }

    let mut perimeters = vec![HashMap::new(); areas.len()];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut surrounds = vec![];
            for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..map.len() as i32).contains(&new_i)
                    || !(0..map[0].len() as i32).contains(&new_j)
                {
                    surrounds.push(std::usize::MAX);
                } else {
                    surrounds.push(group[new_i as usize][new_j as usize]);
                }
            }
            for k in 0..surrounds.len() {
                let d = match k {
                    0 => (i, j + 1),
                    1 => (i + 1, j + 1),
                    2 => (i + 1, j),
                    3 => (i, j),
                    _ => unreachable!(),
                };
                if surrounds[k] != group[i][j]
                    && surrounds[(k + 1) % surrounds.len()] != group[i][j]
                {
                    *perimeters[group[i][j]].entry(d).or_insert(0) += 1;
                    if surrounds[k] != std::usize::MAX
                        && surrounds[k] == surrounds[(k + 1) % surrounds.len()]
                    {
                        *perimeters[surrounds[k]].entry(d).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    (0..areas.len())
        .map(|i| {
            areas[i]
                * perimeters[i]
                    .values()
                    .map(|v| if v == &4 { 2 } else { *v })
                    .sum::<usize>()
        })
        .sum()
}
