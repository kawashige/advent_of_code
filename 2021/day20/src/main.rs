use proconio::input;
use proconio::marker::Chars;

fn solve(algorithm: &[char], original: &Vec<Vec<char>>) -> usize {
    calc(algorithm, original, 2)
}

fn solve2(algorithm: &[char], original: &Vec<Vec<char>>) -> usize {
    calc(algorithm, original, 50)
}

fn calc(algorithm: &[char], original: &Vec<Vec<char>>, times: usize) -> usize {
    let mut image: Vec<Vec<char>> = original.iter().cloned().collect();
    for i in 0..times {
        image = enhance(algorithm, &image, i);
    }

    image
        .iter()
        .map(|row| row.iter().filter(|c| c == &&'#').count())
        .sum()
}

fn enhance(algorithm: &[char], image: &Vec<Vec<char>>, enhanced_count: usize) -> Vec<Vec<char>> {
    let init_char = if algorithm[0] == '.' || enhanced_count % 2 == 1 {
        algorithm[0]
    } else {
        algorithm[511]
    };
    let mut original = vec![vec![init_char; image[0].len() + 4]; image.len() + 4];
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            original[i + 2][j + 2] = image[i][j];
        }
    }
    let mut enhanced = vec![vec![init_char; image[0].len() + 4]; image.len() + 4];

    for i in 0..original.len() {
        for j in 0..original[0].len() {
            let index = if i == 0 || i == original.len() - 1 || j == 0 || j == original[0].len() - 1
            {
                if original[i][j] == '.' {
                    0
                } else {
                    511
                }
            } else {
                convert_index(i, j, &original)
            };
            enhanced[i][j] = algorithm[index];
        }
    }

    enhanced
}

fn convert_index(i: usize, j: usize, image: &Vec<Vec<char>>) -> usize {
    let binary_string = ((i - 1)..=(i + 1))
        .map(|c| {
            ((j - 1)..=(j + 1))
                .map(|r| if image[c][r] == '.' { '0' } else { '1' })
                .collect::<String>()
        })
        .collect::<String>();
    usize::from_str_radix(&binary_string, 2).unwrap()
}

fn main() {
    input! {
        algorithm: Chars,
        n: usize,
        image: [Chars; n]
    }

    println!("part1: {}", solve(&algorithm, &image));
    println!("part2: {}", solve2(&algorithm, &image));
}
