use std::collections::HashSet;
struct Solution {
    tiles: Vec<Vec<String>>,
    moves: Vec<Vec<[i32; 2]>>,
}

impl Solution {
    fn new(s: String) -> Self {
        let mut moves = Vec::new();
        let mut tiles = Vec::new();
        for (m, t) in s.split("\n").map(|mut t| {
            let mut moves = Vec::new();
            let mut tiles = Vec::new();
            while !t.is_empty() {
                if t.starts_with("e") {
                    moves.push([0, 2]);
                    tiles.push("e".to_string());
                    t = t.strip_prefix("e").unwrap();
                } else if t.starts_with("se") {
                    tiles.push("se".to_string());
                    moves.push([1, 1]);
                    t = t.strip_prefix("se").unwrap();
                } else if t.starts_with("sw") {
                    tiles.push("sw".to_string());
                    moves.push([1, -1]);
                    t = t.strip_prefix("sw").unwrap();
                } else if t.starts_with("w") {
                    tiles.push("w".to_string());
                    moves.push([0, -2]);
                    t = t.strip_prefix("w").unwrap();
                } else if t.starts_with("nw") {
                    tiles.push("nw".to_string());
                    moves.push([-1, -1]);
                    t = t.strip_prefix("nw").unwrap();
                } else if t.starts_with("ne") {
                    tiles.push("ne".to_string());
                    moves.push([-1, 1]);
                    t = t.strip_prefix("ne").unwrap();
                }
            }
            (moves, tiles)
        }) {
            moves.push(m);
            tiles.push(t);
        }
        Self { tiles, moves }
    }

    fn identify(moves: &Vec<[i32; 2]>) -> [i32; 2] {
        moves
            .iter()
            .fold([0, 0], |acc, m| [acc[0] + m[0], acc[1] + m[1]])
    }

    fn start_tiles(moves: &Vec<Vec<[i32; 2]>>) -> HashSet<[i32; 2]> {
        let mut set = HashSet::new();
        for m in moves {
            let tile = Self::identify(m);
            if set.contains(&tile) {
                set.remove(&tile);
            } else {
                set.insert(tile);
            }
        }
        set
    }

    fn solve(&self) -> i32 {
        Self::start_tiles(&self.moves).len() as i32
    }

    fn solve2(&self) -> i32 {
        let mut tiles = vec![vec![-1; 1000]; 1000];
        for i in 0..tiles.len() {
            for j in 0..tiles[0].len() {
                if (i % 2 == 0 && j % 2 == 0) || (i % 2 == 1 && j % 2 == 1) {
                    tiles[i][j] = 0;
                }
            }
        }

        for s in Self::start_tiles(&self.moves) {
            tiles[(500 + s[0]) as usize][(500 + s[1]) as usize] = 1;
        }

        for _ in 0..100 {
            let mut counts = vec![vec![0; 1000]; 1000];
            for i in 0..tiles.len() {
                for j in 0..tiles[0].len() {
                    if tiles[i][j] == 1 {
                        for m in vec![[-1_i32, -1_i32], [-1, 1], [0, -2], [0, 2], [1, -1], [1, 1]] {
                            counts[(i as i32 + m[0]) as usize][(j as i32 + m[1]) as usize] += 1;
                        }
                    }
                }
            }
            for i in 0..tiles.len() {
                for j in 0..tiles[0].len() {
                    if tiles[i][j] == 1 && (counts[i][j] == 0 || 2 < counts[i][j]) {
                        tiles[i][j] = 0;
                    } else if tiles[i][j] == 0 && counts[i][j] == 2 {
                        tiles[i][j] = 1;
                    }
                }
            }
        }

        tiles
            .iter()
            .map(|t| t.iter().filter(|tt| *tt == &1).count())
            .sum::<usize>() as i32
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let solution = Solution::new(buf);
    println!("Part1: {}", solution.solve());
    println!("Part2: {}", solution.solve2());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

        let solution = Solution::new(str.to_string());
        assert_eq!(10, solution.solve());
        assert_eq!(2208, solution.solve2());
    }
}
