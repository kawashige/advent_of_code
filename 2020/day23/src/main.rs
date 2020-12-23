struct Solution {
    cups: Vec<u32>,
}

impl Solution {
    fn new(s: &str) -> Self {
        let cups = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        Self { cups }
    }

    fn move_cups(current: usize, cups: Vec<u32>) -> (usize, Vec<u32>) {
        let current_v = cups[current];
        let l = cups.len();
        let mut cups = cups;
        let mut pick_up = if cups.len() - 4 < current {
            let drained = cups.drain((current + 1)..).collect::<Vec<u32>>();
            drained
                .into_iter()
                .chain(cups.drain(..(3 - (l - 1 - current))))
                .collect::<Vec<u32>>()
        } else {
            cups.drain((current + 1)..(current + 4))
                .collect::<Vec<u32>>()
        };
        let insert_pos = (0..cups.len())
            .filter(|i| cups[*i] <= current_v - 1)
            .max_by(|a, b| cups[*a].cmp(&cups[*b]))
            .unwrap_or(
                (0..cups.len())
                    .max_by(|a, b| cups[*a].cmp(&cups[*b]))
                    .unwrap(),
            );
        while !pick_up.is_empty() {
            cups.insert(insert_pos + 1, pick_up.pop().unwrap());
        }
        let next_pos = ((0..cups.len()).find(|i| cups[*i] == current_v).unwrap() + 1) % cups.len();

        (next_pos, cups)
    }

    fn labels(cups: &Vec<u32>) -> u32 {
        let pos = (0..cups.len()).find(|i| cups[*i] == 1).unwrap();
        cups[(pos + 1)..]
            .iter()
            .chain(cups[..pos].iter())
            .map(|c| c.to_string())
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }

    fn solve(&self) -> u32 {
        let mut cups = self.cups.clone();
        let mut current = 0;
        for _ in 0..100 {
            let result = Self::move_cups(current, cups);
            current = result.0;
            cups = result.1;
        }
        Self::labels(&cups)
    }

    fn solve2(&self) -> u64 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for i in 0..(self.cups.len() - 1) {
            map.insert(self.cups[i], self.cups[i + 1]);
        }
        map.insert(*self.cups.last().unwrap(), 10);
        for i in 10..1000000 {
            map.insert(i, i + 1);
        }
        map.insert(1000000, self.cups[0]);

        let mut current = self.cups[0];
        for _ in 0..10000000 {
            let p1 = map[&current];
            let p2 = map[&p1];
            let p3 = map[&p2];

            *map.get_mut(&current).unwrap() = map[&p3];

            let mut dest = current - 1;
            while dest == p1 || dest == p2 || dest == p3 {
                dest -= 1;
            }
            if dest == 0 {
                dest = 1000000;
                while dest == p1 || dest == p2 || dest == p3 {
                    dest -= 1;
                }
            }
            *map.get_mut(&p3).unwrap() = map[&dest];
            *map.get_mut(&dest).unwrap() = p1;

            current = map[&current];
        }

        map[&1] as u64 * map[&map[&1]] as u64
    }
}

fn main() {
    let solution = Solution::new("942387615");
    println!("Part1: {}", solution.solve());
    println!("Part2: {}", solution.solve2());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let solution = Solution::new("389125467");
        assert_eq!(67384529, solution.solve());
        assert_eq!(149245887792, solution.solve2());
    }
}
