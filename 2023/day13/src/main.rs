use std::io::Read;

fn parse_notes(s: String) -> Vec<Vec<Vec<char>>> {
    s.split("\n\n")
        .map(|note| {
            note.split('\n')
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_row_refrection(r: usize, note: &Vec<Vec<char>>) -> bool {
    let len = (r + 1).min(note.len() - (r + 1));
    for i in 0..len {
        for j in 0..note[0].len() {
            if note[r - i][j] != note[r + 1 + i][j] {
                return false;
            }
        }
    }
    true
}
fn is_column_refrection(c: usize, note: &Vec<Vec<char>>) -> bool {
    let len = (c + 1).min(note[0].len() - (c + 1));
    for j in 0..len {
        for i in 0..note.len() {
            if note[i][c - j] != note[i][c + 1 + j] {
                return false;
            }
        }
    }
    true
}

fn is_row_refrection2(r: usize, note: &Vec<Vec<char>>) -> bool {
    let len = (r + 1).min(note.len() - (r + 1));
    let mut count = 0;
    for i in 0..len {
        for j in 0..note[0].len() {
            if note[r - i][j] != note[r + 1 + i][j] {
                count += 1;
            }
        }
    }
    count == 1
}
fn is_column_refrection2(c: usize, note: &Vec<Vec<char>>) -> bool {
    let len = (c + 1).min(note[0].len() - (c + 1));
    let mut count = 0;
    for j in 0..len {
        for i in 0..note.len() {
            if note[i][c - j] != note[i][c + 1 + j] {
                count += 1;
            }
        }
    }
    count == 1
}
fn solve1(notes: &Vec<Vec<Vec<char>>>) -> usize {
    notes
        .iter()
        .map(|note| {
            let mut summary = 0;
            if let Some(i) = (0..note.len() - 1).find(|i| is_row_refrection(*i, note)) {
                summary += (i + 1) * 100;
            }
            if let Some(j) = (0..note[0].len() - 1).find(|j| is_column_refrection(*j, note)) {
                summary += j + 1;
            }
            summary
        })
        .sum()
}

fn solve2(notes: &Vec<Vec<Vec<char>>>) -> usize {
    notes
        .iter()
        .map(|note| {
            let mut summary = 0;
            if let Some(i) = (0..note.len() - 1).find(|i| is_row_refrection2(*i, note)) {
                summary += (i + 1) * 100;
            }
            if let Some(j) = (0..note[0].len() - 1).find(|j| is_column_refrection2(*j, note)) {
                summary += j + 1;
            }
            summary
        })
        .sum()
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let notes = parse_notes(buf);

    println!("{}", solve1(&notes));
    println!("{}", solve2(&notes));
}
