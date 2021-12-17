use proconio::input;

fn solve(_min_x: i32, _max_x: i32, min_y: i32, _max_y: i32) -> i32 {
    (1..min_y.abs()).sum()
}

fn solve2(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i32 {
    (1..=max_x)
        .map(|v_x| {
            (min_y..=min_y.abs())
                .filter(|v_y| is_ok(v_x, *v_y, min_x, max_x, min_y, max_y))
                .count() as i32
        })
        .sum()
}

fn is_ok(v_x1: i32, v_y1: i32, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> bool {
    let (mut v_x, mut v_y) = (v_x1, v_y1);
    let (mut x, mut y) = (0, 0);

    while x <= max_x && min_y <= y {
        if (min_x..=max_x).contains(&x) && (min_y..=max_y).contains(&y) {
            return true;
        }
        x += v_x;
        y += v_y;
        if v_x > 0 {
            v_x -= 1;
        }
        v_y -= 1;
    }

    false
}

fn main() {
    input! {
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32
    }

    println!("part1: {}", solve(min_x, max_x, min_y, max_y));
    println!("part2: {}", solve2(min_x, max_x, min_y, max_y));
}
