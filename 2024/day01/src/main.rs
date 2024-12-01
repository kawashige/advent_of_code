use proconio::input;
fn main() {
    input! {
        n: i32,
        lists: [(i32, i32); n]
    }

    println!("solve1: {:?}", solve1(&lists));
    println!("solve2: {:?}", solve2(&lists));
}

fn solve1(lists: &[(i32, i32)]) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for i in 0..lists.len() {
        left.push(lists[i].0);
        right.push(lists[i].1);
    }
    left.sort_unstable();
    right.sort_unstable();
    (0..lists.len()).map(|i| (left[i] - right[i]).abs()).sum()
}

fn solve2(lists: &[(i32, i32)]) -> i32 {
    let mut result = 0;
    for i in 0..lists.len() {
        let mut count = 0;
        for j in 0..lists.len() {
            if lists[i].0 == lists[j].1 {
                count += 1;
            }
        }
        result += lists[i].0 * count;
    }
    result
}
