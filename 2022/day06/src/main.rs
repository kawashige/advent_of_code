use proconio::input;
use proconio::marker::Chars;

fn find_different_character_index(datasteam: &[char], count: usize) -> usize {
    (count..datasteam.len())
        .find(|i| {
            datasteam[i - count..*i]
                .iter()
                .fold(0_usize, |acc, c| acc | 1 << *c as u8 - b'a')
                .count_ones() as usize
                == count
        })
        .unwrap()
}
fn solve1(datasteam: &[char]) -> usize {
    find_different_character_index(&datasteam, 4)
}

fn solve2(datasteam: &[char]) -> usize {
    find_different_character_index(&datasteam, 14)
}

fn main() {
    input! {
        datastream: Chars
    }

    println!("{}", solve1(&datastream));
    println!("{}", solve2(&datastream));
}
