struct Solution {
    images: HashMap<i32, Vec<Vec<usize>>>,
}

use std::collections::HashMap;
impl Solution {
    fn new(s: String) -> Self {
        let images = s
            .split("\n\n")
            .map(|tile| {
                let mut splitted = tile.split('\n');
                let tile_no = splitted
                    .next()
                    .unwrap()
                    .trim_start_matches("Tile ")
                    .trim_end_matches(":")
                    .parse::<i32>()
                    .unwrap();
                let image = splitted
                    .map(|l| {
                        l.chars()
                            .map(|c| if c == '#' { 1 } else { 0 })
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>();
                (tile_no, image)
            })
            .collect::<HashMap<i32, Vec<Vec<usize>>>>();
        Self { images }
    }

    fn rotate_90(input: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut output = vec![vec![0; input[0].len()]; input.len()];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                output[j][input[0].len() - 1 - i] = input[i][j];
            }
        }
        output
    }

    fn flip_vertical(input: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut output = vec![vec![0; input[0].len()]; input.len()];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                output[i][input[0].len() - 1 - j] = input[i][j];
            }
        }
        output
    }

    fn flip_horizontal(input: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut output = vec![vec![0; input[0].len()]; input.len()];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                output[input.len() - 1 - i][j] = input[i][j];
            }
        }
        output
    }

    fn find_sea_monster(image: &Vec<Vec<usize>>) -> Option<i32> {
        let points = [
            [-1_i32, 18],
            [0, 5],
            [0, 6],
            [0, 11],
            [0, 12],
            [0, 17],
            [0, 18],
            [0, 19],
            [1, 1],
            [1, 4],
            [1, 7],
            [1, 10],
            [1, 13],
            [1, 16],
        ];
        let mut sea_monster = 0;
        for i in 1..(image.len() - 1) {
            for j in 0..(image.len() - 18) {
                if image[i][j] == 1
                    && points
                        .iter()
                        .all(|p| image[(p[0] + i as i32) as usize][(p[1] + j as i32) as usize] == 1)
                {
                    sea_monster += 1;
                }
            }
        }
        if sea_monster == 0 {
            None
        } else {
            Some(
                (image.iter().map(|l| l.iter().sum::<usize>()).sum::<usize>()
                    - sea_monster * (points.len() + 1)) as i32,
            )
        }
    }

    fn solve(&self) -> i64 {
        let keys = self.images.keys().cloned().collect::<Vec<i32>>();
        let images = keys
            .iter()
            .map(|k| self.images[k].clone())
            .collect::<Vec<Vec<Vec<usize>>>>();

        let l = images[0].len();
        let mut result = 1 as i64;
        for i in 0..keys.len() {
            let mut edge_count = vec![0; 4];
            for j in 0..keys.len() {
                if i == j {
                    continue;
                }
                // Top
                if (0..l).all(|k| images[i][0][k] == images[j][0][k])
                    || (0..l).all(|k| images[i][0][k] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][0][k] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][0][k] == images[j][k][0])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1 - k][0])
                {
                    edge_count[0] += 1;
                }
                // Right
                if (0..l).all(|k| images[i][k][l - 1] == images[j][0][k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][k][0])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1 - k][0])
                {
                    edge_count[1] += 1;
                }
                // Bottom
                if (0..l).all(|k| images[i][l - 1][k] == images[j][0][k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][k][0])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1 - k][0])
                {
                    edge_count[2] += 1;
                }
                // Left
                if (0..l).all(|k| images[i][k][0] == images[j][0][k])
                    || (0..l).all(|k| images[i][k][0] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][k][0] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][k][0] == images[j][k][0])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1 - k][0])
                {
                    edge_count[3] += 1;
                }
            }
            if edge_count.iter().filter(|l| *l == &0).count() == 2 {
                result *= keys[i] as i64;
            }
        }
        result
    }

    fn solve2(&self) -> i32 {
        let keys = self.images.keys().cloned().collect::<Vec<i32>>();
        let images = keys
            .iter()
            .map(|k| self.images[k].clone())
            .collect::<Vec<Vec<Vec<usize>>>>();

        let l = images[0].len();
        let mut all_edges = Vec::new();
        for i in 0..keys.len() {
            let mut edges = vec![vec![] as Vec<usize>; 4];
            for j in 0..keys.len() {
                if i == j {
                    continue;
                }
                // Top
                if (0..l).all(|k| images[i][0][k] == images[j][0][k])
                    || (0..l).all(|k| images[i][0][k] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][0][k] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][0][k] == images[j][k][0])
                    || (0..l).all(|k| images[i][0][k] == images[j][l - 1 - k][0])
                {
                    edges[0].push(j);
                }
                // Right
                if (0..l).all(|k| images[i][k][l - 1] == images[j][0][k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][k][0])
                    || (0..l).all(|k| images[i][k][l - 1] == images[j][l - 1 - k][0])
                {
                    edges[1].push(j);
                }
                // Bottom
                if (0..l).all(|k| images[i][l - 1][k] == images[j][0][k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][k][0])
                    || (0..l).all(|k| images[i][l - 1][k] == images[j][l - 1 - k][0])
                {
                    edges[2].push(j);
                }
                // Left
                if (0..l).all(|k| images[i][k][0] == images[j][0][k])
                    || (0..l).all(|k| images[i][k][0] == images[j][0][l - 1 - k])
                    || (0..l).all(|k| images[i][k][0] == images[j][k][l - 1])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1 - k][l - 1])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1][k])
                    || (0..l).all(|k| images[i][k][0] == images[j][l - 1][l - 1 - k])
                    || (0..l).all(|k| images[i][k][0] == images[j][k][0])
                {
                    edges[3].push(j);
                }
            }
            all_edges.push(edges);
        }

        let ll = (self.images.len() as f32).sqrt() as usize;
        let mut image_ids = vec![vec![0; ll]; ll];
        let top_left = all_edges
            .iter()
            .enumerate()
            .find(|(_, e)| e[0].is_empty() && e[3].is_empty())
            .unwrap();
        image_ids[0][0] = top_left.0;

        let ll = (self.images.len() as f32).sqrt() as usize;
        let mut whole_image = vec![vec![0; (l - 2) * ll]; (l - 2) * ll];

        for i in 0..ll {
            for j in 0..ll {
                let mut edge = all_edges[image_ids[i][j]].clone();
                let mut image = images[image_ids[i][j]].clone();
                if i == 0 && j != 0 {
                    while edge[3].is_empty() || edge[3][0] != image_ids[i][j - 1] {
                        edge.rotate_right(1);
                        image = Self::rotate_90(&image);
                    }
                    if !edge[0].is_empty() {
                        edge.swap(0, 2);
                        image = Self::flip_horizontal(&image);
                    }
                } else if i != 0 && j == 0 {
                    while edge[0].is_empty() || edge[0][0] != image_ids[i - 1][j] {
                        edge.rotate_right(1);
                        image = Self::rotate_90(&image);
                    }
                    if !edge[3].is_empty() {
                        edge.swap(1, 3);
                        image = Self::flip_vertical(&image);
                    }
                } else if i != 0 && j != 0 {
                    while edge[0].is_empty() || edge[0][0] != image_ids[i - 1][j] {
                        edge.rotate_right(1);
                        image = Self::rotate_90(&image);
                    }
                    if edge[3].is_empty() || edge[3][0] != image_ids[i][j - 1] {
                        edge.swap(1, 3);
                        image = Self::flip_vertical(&image);
                    }
                }
                if !edge[1].is_empty() {
                    image_ids[i][j + 1] = edge[1][0];
                }
                if !edge[2].is_empty() {
                    image_ids[i + 1][j] = edge[2][0];
                }

                let offset_x = i * (l - 2);
                let offset_y = j * (l - 2);
                for k in 1..(image.len() - 1) {
                    for l in 1..(image[0].len() - 1) {
                        whole_image[offset_x + k - 1][offset_y + l - 1] = image[k][l]
                    }
                }
            }
        }

        for _ in 0..4 {
            let result = Self::find_sea_monster(&whole_image);
            if result.is_some() {
                return result.unwrap();
            }
            let result = Self::find_sea_monster(&Self::flip_horizontal(&whole_image));
            if result.is_some() {
                return result.unwrap();
            }
            let result = Self::find_sea_monster(&Self::flip_vertical(&whole_image));
            if result.is_some() {
                return result.unwrap();
            }
            whole_image = Self::rotate_90(&whole_image);
        }

        0
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
        let str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        let solution = Solution::new(str.to_string());
        assert_eq!(20899048083289, solution.solve());
        assert_eq!(273, solution.solve2());
    }
}
