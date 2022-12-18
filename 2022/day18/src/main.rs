use std::collections::VecDeque;

use proconio::input;

fn is_connected(cube1: &[i32], cube2: &[i32]) -> bool {
    (cube1[0] == cube2[0] && cube1[1] == cube2[1] && (cube1[2] - cube2[2]).abs() == 1)
        || (cube1[1] == cube2[1] && cube1[2] == cube2[2] && (cube1[0] - cube2[0]).abs() == 1)
        || (cube1[0] == cube2[0] && cube1[2] == cube2[2] && (cube1[1] - cube2[1]).abs() == 1)
}

fn solve1(cubes: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    for i in 0..cubes.len() {
        count += 6
            - (0..cubes.len())
                .filter(|j| is_connected(&cubes[i], &cubes[*j]))
                .count();
    }

    count
}

fn solve2(cubes: &Vec<Vec<i32>>) -> usize {
    let max_coodinate = *cubes.iter().flatten().max().unwrap() as usize + 3;
    let mut grid = vec![vec![vec![0; max_coodinate]; max_coodinate]; max_coodinate];
    for cube in cubes {
        grid[cube[0] as usize + 1][cube[1] as usize + 1][cube[2] as usize + 1] = 1;
    }
    let mut queue = VecDeque::new();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            queue.push_back((0, i, j));
            queue.push_back((max_coodinate - 1, i, j));
            queue.push_back((i, 0, j));
            queue.push_back((i, max_coodinate - 1, j));
            queue.push_back((i, j, 0));
            queue.push_back((i, j, max_coodinate - 1));
        }
    }
    while let Some((i, j, k)) = queue.pop_front() {
        if grid[i][j][k] == 2 {
            continue;
        }
        grid[i][j][k] = 2;
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        {
            let (x, y, z) = (i as i32 + dx, j as i32 + dy, k as i32 + dz);
            if x < 0
                || y < 0
                || z < 0
                || max_coodinate as i32 <= x
                || max_coodinate as i32 <= y
                || max_coodinate as i32 <= z
                || grid[x as usize][y as usize][z as usize] != 0
            {
                continue;
            }
            queue.push_back((x as usize, y as usize, z as usize));
        }
    }

    let mut count = 0;

    for i in 0..cubes.len() {
        count += [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        .filter(|(dx, dy, dz)| {
            grid[(cubes[i][0] as i32 + 1 + dx) as usize][(cubes[i][1] as i32 + 1 + dy) as usize]
                [(cubes[i][2] as i32 + 1 + dz) as usize]
                == 2
        })
        .count()
    }

    count
}

fn main() {
    input! {
        n: usize,
        cubes: [String; n],
    }

    let cubes = cubes
        .into_iter()
        .map(|cube| {
            cube.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&cubes));
    println!("{}", solve2(&cubes));
}
