use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let schematics = buf
        .split("\n\n")
        .map(|locks| {
            locks
                .split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&schematics));
}

fn solve1(schematics: &Vec<Vec<Vec<char>>>) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for i in 0..schematics.len() {
        if schematics[i][0].iter().all(|c| c == &'#') {
            let lock = (0..schematics[i][0].len())
                .map(|c| {
                    (0..schematics[i].len())
                        .find(|r| schematics[i][*r][c] == '.')
                        .unwrap()
                        - 1
                })
                .collect::<Vec<_>>();
            locks.push(lock);
        } else {
            let key = (0..schematics[i][0].len())
                .map(|c| {
                    schematics[i].len()
                        - 1
                        - (0..schematics[i].len())
                            .find(|r| schematics[i][*r][c] == '#')
                            .unwrap()
                })
                .collect::<Vec<_>>();
            keys.push(key);
        }
    }

    let mut count = 0;
    for i in 0..locks.len() {
        for j in 0..keys.len() {
            if (0..locks[i].len()).all(|k| locks[i][k] + keys[j][k] <= schematics[0].len() - 2) {
                count += 1;
            }
        }
    }
    count
}
