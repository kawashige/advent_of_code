use proconio::input;
use proconio::marker::Chars;

fn solve(heightmap: &Vec<Vec<char>>) -> usize {
    let low_points = find_low_points(heightmap);
    low_points
        .into_iter()
        .map(|(i, j)| heightmap[i][j].to_digit(10).unwrap() as usize + 1)
        .sum()
}

fn solve2(heightmap: &Vec<Vec<char>>) -> usize {
    let low_points = find_low_points(heightmap);
    let mut basins = low_points
        .into_iter()
        .map(|low_point| get_basin_size(heightmap, low_point))
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins.into_iter().rev().take(3).product()
}

fn find_low_points(heightmap: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    (0..heightmap.len())
        .flat_map(|i| {
            (0..heightmap[0].len())
                .filter(|j| {
                    [(0, -1), (-1, 0), (0, 1), (1, 0)].iter().all(|(dx, dy)| {
                        let (x, y) = (i as i32 + dx, *j as i32 + dy);
                        x < 0
                            || heightmap.len() as i32 <= x
                            || y < 0
                            || heightmap[0].len() as i32 <= y
                            || heightmap[i][*j] < heightmap[x as usize][y as usize]
                    })
                })
                .map(|j| (i, j))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_basin_size(heightmap: &Vec<Vec<char>>, low_point: (usize, usize)) -> usize {
    let mut seen = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    let mut stack = vec![low_point];
    let mut size = 0;

    while let Some((i, j)) = stack.pop() {
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;
        size += 1;

        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)].iter() {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0
                || heightmap.len() as i32 <= x
                || y < 0
                || heightmap[0].len() as i32 <= y
                || seen[x as usize][y as usize]
                || heightmap[x as usize][y as usize] == '9'
                || heightmap[i][j] >= heightmap[x as usize][y as usize]
            {
                continue;
            }
            stack.push((x as usize, y as usize));
        }
    }
    size
}

fn main() {
    input! {
        n: usize,
        heightmap: [Chars; n]
    }

    println!("part1: {}", solve(&heightmap));
    println!("part2: {}", solve2(&heightmap));
}
