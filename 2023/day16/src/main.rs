use proconio::input;
use proconio::marker::Chars;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_nexe_move(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn get_next_direction(point: char, direction: Direction) -> Vec<Direction> {
    match point {
        '/' => match direction {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        '\\' => match direction {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
        '-' if direction == Direction::Up || direction == Direction::Down => {
            vec![Direction::Left, Direction::Right]
        }
        '|' if direction == Direction::Left || direction == Direction::Right => {
            vec![Direction::Up, Direction::Down]
        }
        _ => vec![direction],
    }
}

fn find(start_pos: (i32, i32), start_direction: Direction, layout: &Vec<Vec<char>>) -> usize {
    let mut seen = vec![vec![vec![false; 4]; layout[0].len()]; layout.len()];
    let mut beam = vec![(start_pos, start_direction)];
    let mut energized = vec![vec![false; layout[0].len()]; layout.len()];

    while let Some(((i, j), direction)) = beam.pop() {
        if seen[i as usize][j as usize][direction as usize] {
            continue;
        }
        seen[i as usize][j as usize][direction as usize] = true;
        energized[i as usize][j as usize] = true;
        for next_direction in get_next_direction(layout[i as usize][j as usize], direction) {
            let next_move = get_nexe_move(next_direction);
            let (next_i, next_j) = (i + next_move.0, j + next_move.1);
            if !(0..layout.len() as i32).contains(&next_i)
                || !(0..layout[0].len() as i32).contains(&next_j)
                || seen[next_i as usize][next_j as usize][next_direction as usize]
            {
                continue;
            }
            beam.push(((next_i, next_j), next_direction));
        }
    }

    energized
        .into_iter()
        .map(|row| row.into_iter().filter(|i| *i).count())
        .sum()
}

fn solve1(layout: &Vec<Vec<char>>) -> usize {
    find((0, 0), Direction::Right, layout)
}

fn solve2(layout: &Vec<Vec<char>>) -> usize {
    let mut max = 0;
    for i in 0..layout.len() as i32 {
        max = max.max(find((i, 0), Direction::Right, layout));
        max = max.max(find(
            (i, layout[0].len() as i32 - 1),
            Direction::Left,
            layout,
        ));
    }
    for j in 0..layout[0].len() as i32 {
        max = max.max(find((0, j), Direction::Down, layout));
        max = max.max(find((layout.len() as i32 - 1, j), Direction::Up, layout));
    }
    max
}
fn main() {
    input! {
        n: usize,
        layout: [Chars; n]
    }

    println!("{}", solve1(&layout));
    println!("{}", solve2(&layout));
}
