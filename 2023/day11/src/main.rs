use proconio::input;
use proconio::marker::Chars;

fn solve1(image: &Vec<Vec<char>>) -> usize {
    let mut galaxies = Vec::new();
    let mut row = vec![1; image.len()];
    let mut column = vec![1; image[0].len()];
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if image[i][j] == '#' {
                galaxies.push((i, j));
                row[i] = 0;
                column[j] = 0;
            }
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result += (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + row[galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0)]
                    .iter()
                    .sum::<i32>()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs()
                + column[galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1)]
                    .iter()
                    .sum::<i32>();
        }
    }
    result as usize
}

fn solve2(image: &Vec<Vec<char>>) -> usize {
    let mut galaxies = Vec::new();
    let mut row = vec![1; image.len()];
    let mut column = vec![1; image[0].len()];
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if image[i][j] == '#' {
                galaxies.push((i, j));
                row[i] = 0;
                column[j] = 0;
            }
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result += (galaxies[i].0 as i128 - galaxies[j].0 as i128).abs()
                + row[galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0)]
                    .iter()
                    .sum::<i128>()
                    * 999999
                + (galaxies[i].1 as i128 - galaxies[j].1 as i128).abs()
                + column[galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1)]
                    .iter()
                    .sum::<i128>()
                    * 999999;
        }
    }
    result as usize
}
fn main() {
    input! {
        n: usize,
        image: [Chars; n]
    }

    println!("{}", solve1(&image));
    println!("{}", solve2(&image));
}
