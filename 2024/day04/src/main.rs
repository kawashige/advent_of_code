use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        word_search: [Chars; n]
    }

    println!("{:?}", solve1(&word_search));
    println!("{:?}", solve2(&word_search));
}

fn solve1(word_search: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let target = "MAS".chars().collect::<Vec<_>>();

    for i in 0..word_search.len() {
        for j in 0..word_search[0].len() {
            if word_search[i][j] != 'X' {
                continue;
            }

            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            {
                let mut found = false;
                let (mut new_i, mut new_j) = (i as i32, j as i32);
                for k in 0..target.len() {
                    new_i += di;
                    new_j += dj;
                    if !(0..word_search.len() as i32).contains(&new_i)
                        || !(0..word_search[0].len() as i32).contains(&new_j)
                        || word_search[new_i as usize][new_j as usize] != target[k]
                    {
                        break;
                    }
                    if k == target.len() - 1 {
                        found = true;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve2(word_search: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let target = [
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
    ];

    for i in 0..word_search.len() - target[0].len() + 1 {
        for j in 0..word_search[0].len() - target[0][0].len() + 1 {
            for t in 0..target.len() {
                let mut found = true;
                for x in 0..target[0].len() {
                    if !found {
                        break;
                    }
                    for y in 0..target[0][0].len() {
                        if target[t][x][y] == '.' || word_search[i + x][j + y] == target[t][x][y] {
                            continue;
                        }
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}
