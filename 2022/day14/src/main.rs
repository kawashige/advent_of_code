use std::io::Read;

fn fall(coodinate: (usize, usize), grid: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
    let (mut x, mut y) = coodinate;
    while (0..grid.len() - 1).contains(&x) && (1..grid[0].len()).contains(&y) {
        if let Some((dx, dy)) = [(1, 0), (1, -1), (1, 1)]
            .iter()
            .find(|(dx, dy)| grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize] == 0)
        {
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;
        } else {
            return Some((x, y));
        }
    }
    None
}

fn solve1(grid: &Vec<Vec<usize>>) -> usize {
    let mut grid = grid.clone();
    let init = (0, 500);
    let mut count = 0;

    while let Some((x, y)) = fall(init, &grid) {
        grid[x][y] = 2;
        count += 1
    }

    count
}

fn solve2(grid: &Vec<Vec<usize>>) -> usize {
    let mut grid = grid.clone();
    grid.push(vec![0; grid[0].len()]);
    grid.push(vec![1; grid[0].len()]);
    let init = (0, 500);
    let mut count = 0;

    while let Some((x, y)) = fall(init, &grid) {
        grid[x][y] = 2;
        count += 1;
        if (x, y) == init {
            break;
        }
    }

    count
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let rocks = buf
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|cood| {
                    cood.split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_coodinate = rocks.iter().flatten().fold((0, 0), |(max_x, max_y), cood| {
        (max_x.max(cood[0]), max_y.max(cood[1]))
    });

    let mut grid = vec![vec![0; 1000]; max_coodinate.1 + 1];
    for rock in rocks {
        for i in 1..rock.len() {
            for x in rock[i][1].min(rock[i - 1][1])..=rock[i][1].max(rock[i - 1][1]) {
                for y in rock[i][0].min(rock[i - 1][0])..=rock[i][0].max(rock[i - 1][0]) {
                    grid[x][y] = 1;
                }
            }
        }
    }

    println!("{}", solve1(&grid));
    println!("{}", solve2(&grid));
}
