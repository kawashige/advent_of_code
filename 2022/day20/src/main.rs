use proconio::input;

fn solve1(coodinates: &[i32]) -> i32 {
    let mut result = coodinates.iter().cloned().enumerate().collect::<Vec<_>>();
    for i in 0..coodinates.len() {
        let mut j = result.iter().position(|(j, _)| j == &i).unwrap();
        let move_count =
            (coodinates[i].abs() % (coodinates.len() as i32 - 1)) * coodinates[i].signum();
        for _ in 0..move_count.abs() {
            let target =
                ((j as i32 + move_count.signum() + result.len() as i32) as usize) % result.len();
            result.swap(j, target);
            j = target;
        }
    }
    let i = result.iter().position(|(_, x)| x == &0).unwrap();

    result[(i + 1000) % result.len()].1
        + result[(i + 2000) % result.len()].1
        + result[(i + 3000) % result.len()].1
}

fn solve2(coodinates: &[i32]) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut result = coodinates
        .iter()
        .map(|x| *x as i64 * DECRYPTION_KEY)
        .enumerate()
        .collect::<Vec<_>>();
    for _ in 0..10 {
        for i in 0..coodinates.len() {
            let mut j = result.iter().position(|(j, _)| j == &i).unwrap();
            let move_count = (result[j].1.abs() % (result.len() as i64 - 1)) * result[j].1.signum();
            for _ in 0..move_count.abs() {
                let target = ((j as i64 + move_count.signum() + result.len() as i64) as usize)
                    % result.len();
                result.swap(j, target);
                j = target;
            }
        }
    }
    let i = result.iter().position(|(_, x)| x == &0).unwrap();

    result[(i + 1000) % result.len()].1
        + result[(i + 2000) % result.len()].1
        + result[(i + 3000) % result.len()].1
}

fn main() {
    input! {
        n: usize,
        coodinates: [i32; n]
    }
    println!("{}", solve1(&coodinates));
    println!("{}", solve2(&coodinates));
}
