struct Solution {
    card_public_key: i64,
    door_public_key: i64,
}

impl Solution {
    fn new(card_public_key: i64, door_public_key: i64) -> Self {
        Self {
            card_public_key,
            door_public_key,
        }
    }

    fn find_loop_size(key: i64) -> i64 {
        let mut v = 1;
        let mut result = 0;
        while v != key {
            v *= 7;
            v %= 20201227;
            result += 1;
        }
        result
    }

    fn transform(loop_size: i64, subject_number: i64) -> i64 {
        let mut v = 1;
        for _ in 0..loop_size {
            v *= subject_number;
            v %= 20201227;
        }
        v
    }

    fn solve(&self) -> i64 {
        let card_loop_size = Self::find_loop_size(self.card_public_key);
        Self::transform(card_loop_size, self.door_public_key)
    }
}

fn main() {
    let solution = Solution::new(12090988, 240583);
    println!("Part1: {}", solution.solve());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let solution = Solution::new(5764801, 17807724);
        assert_eq!(14897079, solution.solve());
    }
}
