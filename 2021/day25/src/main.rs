use proconio::input;
use proconio::marker::Chars;

fn solve(area: &Vec<Vec<char>>) -> usize {
    let mut area: Vec<Vec<char>> = area.into_iter().cloned().collect();
    let rows = area.len();
    let columns = area[0].len();
    let mut moved = true;
    let mut state = 1;

    while moved {
        moved = false;
        let mut new_area = vec![vec!['.'; columns]; rows];

        for i in 0..rows {
            for j in 0..columns {
                if area[i][j] == '>' {
                    if area[i][(j + 1) % columns] == '.' {
                        moved = true;
                        new_area[i][(j + 1) % columns] = '>';
                    } else {
                        new_area[i][j] = '>';
                    }
                }
            }
        }

        for i in 0..rows {
            for j in 0..columns {
                if area[i][j] == 'v' {
                    if area[(i + 1) % rows][j] != 'v' && new_area[(i + 1) % rows][j] == '.' {
                        moved = true;
                        new_area[(i + 1) % rows][j] = 'v';
                    } else {
                        new_area[i][j] = 'v';
                    }
                }
            }
        }

        if moved {
            state += 1;
        }

        area = new_area;
    }
    state
}

fn main() {
    input! {
        n: usize,
        area: [Chars; n]
    }

    println!("part1: {}", solve(&area));
}
