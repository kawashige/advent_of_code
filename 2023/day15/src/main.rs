use std::io::Read;

fn parse_sequences(s: String) -> Vec<String> {
    s.split(',').map(|s| s.to_string()).collect()
}

fn hash_algorithm(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve1(sequences: &Vec<String>) -> usize {
    sequences.iter().map(|s| hash_algorithm(s)).sum()
}
fn solve2(sequences: &Vec<String>) -> usize {
    let mut boxes = vec![vec![]; 256];
    for s in sequences {
        if s.ends_with('-') {
            let label = s[..s.len() - 1].to_string();
            let label_hash = hash_algorithm(&label);
            boxes[label_hash] = boxes[label_hash]
                .iter()
                .cloned()
                .filter(|(l, _)| l != &label)
                .collect();
        } else {
            let mut sp = s.split('=');
            let label = sp.next().unwrap().to_string();
            let label_hash = hash_algorithm(&label);
            let value = sp.next().unwrap().parse::<usize>().unwrap();
            let mut found = false;
            for i in 0..boxes[label_hash].len() {
                if boxes[label_hash][i].0 == label {
                    boxes[label_hash][i].1 = value;
                    found = true;
                }
            }
            if !found {
                boxes[label_hash].push((label, value));
            }
        }
    }

    let mut power = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            power += (i + 1) * (j + 1) * boxes[i][j].1;
        }
    }
    power
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let sequences = parse_sequences(buf);

    println!("{:?}", sequences);

    println!("{}", solve1(&sequences));
    println!("{}", solve2(&sequences));
}
