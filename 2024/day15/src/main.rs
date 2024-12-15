use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (map, movement) = {
        let mut sp = buf.split("\n\n");
        let map = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let movement = sp
            .next()
            .unwrap()
            .replace("\n", "")
            .chars()
            .collect::<Vec<_>>();
        (map, movement)
    };

    println!("{:?}", solve1(&map, &movement));
    println!("{:?}", solve2(&map, &movement));
}

fn solve1(map_: &Vec<Vec<char>>, movement: &Vec<char>) -> usize {
    let mut pos = (0, 0);
    let mut map = map_.clone();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                pos = (i, j);
                map[i][j] = '.';
                break;
            }
        }
    }

    let d = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for m in movement {
        let d = d[match m {
            '^' => 0,
            '<' => 1,
            '>' => 2,
            'v' => 3,
            _ => unreachable!(),
        }];
        let new_pos = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize);
        if map[new_pos.0][new_pos.1] == 'O' {
            let mut p = new_pos;
            while map[p.0][p.1] == 'O' {
                p.0 = (p.0 as i32 + d.0) as usize;
                p.1 = (p.1 as i32 + d.1) as usize;
            }
            if map[p.0][p.1] == '.' {
                map[p.0][p.1] = 'O';
                map[new_pos.0][new_pos.1] = '.';
            }
        }
        if map[new_pos.0][new_pos.1] == '.' {
            pos = new_pos;
        }
    }

    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                result += i * 100 + j;
            }
        }
    }
    result
}

fn solve2(map_: &Vec<Vec<char>>, movement: &Vec<char>) -> usize {
    let mut map = map_
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| match c {
                    &'#' => vec!['#', '#'],
                    &'O' => vec!['[', ']'],
                    &'.' => vec!['.', '.'],
                    &'@' => vec!['@', '.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                pos = (i, j);
                map[i][j] = '.';
                break;
            }
        }
    }

    let d = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for m in movement {
        let d = d[match m {
            '^' => 0,
            '<' => 1,
            '>' => 2,
            'v' => 3,
            _ => unreachable!(),
        }];
        let new_pos = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize);
        if m == &'<' {
            if map[new_pos.0][new_pos.1] == ']' {
                let mut j = new_pos.1;
                while map[new_pos.0][j] == ']' {
                    j -= 2;
                }
                if map[new_pos.0][j] == '.' {
                    for k in j..new_pos.1 {
                        map[new_pos.0][k] = if k % 2 == j % 2 { '[' } else { ']' };
                    }
                    map[new_pos.0][new_pos.1] = '.';
                }
            }
            if map[new_pos.0][new_pos.1] == '.' {
                pos = new_pos;
            }
        } else if m == &'>' {
            if map[new_pos.0][new_pos.1] == '[' {
                let mut j = new_pos.1;
                while map[new_pos.0][j] == '[' {
                    j += 2;
                }
                if map[new_pos.0][j] == '.' {
                    for k in new_pos.1 + 1..=j {
                        map[new_pos.0][k] = if k % 2 == (new_pos.1 + 1) % 2 {
                            '['
                        } else {
                            ']'
                        };
                    }
                    map[new_pos.0][new_pos.1] = '.';
                }
            }
            if map[new_pos.0][new_pos.1] == '.' {
                pos = new_pos;
            }
        } else if m == &'^' {
            if map[new_pos.0][new_pos.1] == '[' || map[new_pos.0][new_pos.1] == ']' {
                let mut move_target = Vec::new();
                let mut indices = HashSet::new();
                indices.insert(new_pos.1);
                indices.insert(
                    (new_pos.1 as i32
                        + if map[new_pos.0][new_pos.1] == '[' {
                            1
                        } else {
                            -1
                        }) as usize,
                );
                let mut i = 1;
                let mut can_move = true;
                loop {
                    let mut new_indices = HashSet::new();
                    for j in &indices {
                        move_target.push((new_pos.0 - i + 1, *j));
                        if map[new_pos.0 - i][*j] == '#' {
                            can_move = false;
                            break;
                        }
                        if !can_move {
                            break;
                        }
                        if map[new_pos.0 - i][*j] == '[' {
                            new_indices.insert(*j);
                            new_indices.insert(*j + 1);
                        }
                        if map[new_pos.0 - i][*j] == ']' {
                            new_indices.insert(*j);
                            new_indices.insert(*j - 1);
                        }
                    }
                    if new_indices.is_empty() {
                        break;
                    }
                    indices = new_indices;
                    i += 1;
                }
                if can_move {
                    for (i, j) in move_target.into_iter().rev() {
                        map[i - 1][j] = map[i][j];
                        map[i][j] = '.'
                    }
                }
            }
        } else {
            if map[new_pos.0][new_pos.1] == '[' || map[new_pos.0][new_pos.1] == ']' {
                let mut move_target = Vec::new();
                let mut indices = HashSet::new();
                indices.insert(new_pos.1);
                indices.insert(
                    (new_pos.1 as i32
                        + if map[new_pos.0][new_pos.1] == '[' {
                            1
                        } else {
                            -1
                        }) as usize,
                );
                let mut i = 1;
                let mut can_move = true;
                loop {
                    let mut new_indices = HashSet::new();
                    for j in &indices {
                        move_target.push((new_pos.0 + i - 1, *j));
                        if map[new_pos.0 + i][*j] == '#' {
                            can_move = false;
                            break;
                        }
                        if !can_move {
                            break;
                        }
                        if map[new_pos.0 + i][*j] == '[' {
                            new_indices.insert(*j);
                            new_indices.insert(*j + 1);
                        }
                        if map[new_pos.0 + i][*j] == ']' {
                            new_indices.insert(*j);
                            new_indices.insert(*j - 1);
                        }
                    }
                    if new_indices.is_empty() {
                        break;
                    }
                    indices = new_indices;
                    i += 1;
                }
                if can_move {
                    for (i, j) in move_target.into_iter().rev() {
                        map[i + 1][j] = map[i][j];
                        map[i][j] = '.'
                    }
                }
            }
        }
        if map[new_pos.0][new_pos.1] == '.' {
            pos = new_pos;
        }
    }

    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '[' {
                result += i * 100 + j;
            }
        }
    }
    result
}
