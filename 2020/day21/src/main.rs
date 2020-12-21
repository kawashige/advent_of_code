struct Solution {
    ingredients: Vec<Vec<String>>,
    allergens: Vec<Vec<String>>,
}

use std::collections::{BTreeMap, HashMap, HashSet};
impl Solution {
    fn new(s: String) -> Self {
        let mut ingredients = Vec::new();
        let mut allergens = Vec::new();
        for (i, a) in s.split('\n').map(|l| {
            let mut splitted = l.split(" (contains ");
            let ingredients = splitted
                .next()
                .unwrap()
                .split(' ')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            let allergens = splitted
                .next()
                .unwrap()
                .trim_end_matches(')')
                .split(", ")
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            (ingredients, allergens)
        }) {
            ingredients.push(i);
            allergens.push(a)
        }
        Self {
            ingredients,
            allergens,
        }
    }

    fn make_map(&self) -> HashMap<String, HashSet<String>> {
        let mut map = HashMap::new();
        for i in 0..self.ingredients.len() {
            for j in 0..self.allergens[i].len() {
                let candidates = self.ingredients[i]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>();
                if map.contains_key(&self.allergens[i][j]) {
                    *map.get_mut(&self.allergens[i][j]).unwrap() = candidates
                        .intersection(&map[&self.allergens[i][j]])
                        .cloned()
                        .collect::<HashSet<String>>()
                } else {
                    map.insert(self.allergens[i][j].to_string(), candidates);
                }
            }
        }
        let mut decided = HashSet::new();
        let count = map.len();
        let keys = map.keys().map(|k| k.to_string()).collect::<Vec<String>>();
        while decided.len() < count {
            for k in &keys {
                if 1 < map[k].len() {
                    *map.get_mut(k).unwrap() = map[k]
                        .difference(&decided)
                        .cloned()
                        .collect::<HashSet<String>>();
                }
                if map[k].len() == 1 {
                    let v = map[k].iter().next().unwrap().to_string();
                    if !decided.contains(&v) {
                        decided.insert(v);
                    }
                }
            }
        }
        map
    }

    fn solve(&self) -> i32 {
        let map = self.make_map();
        let has_allergens = map
            .values()
            .map(|v| v.iter().next().unwrap().to_string())
            .collect::<HashSet<String>>();

        self.ingredients
            .iter()
            .map(|i| i.iter().filter(|ii| !has_allergens.contains(*ii)).count())
            .sum::<usize>() as i32
    }

    fn solve2(&self) -> String {
        self.make_map()
            .into_iter()
            .collect::<BTreeMap<String, HashSet<String>>>()
            .into_iter()
            .map(|(_, v)| v.iter().next().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(",")
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
        let str1 = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        let solution = Solution::new(str1.to_string());
        assert_eq!(5, solution.solve());
        assert_eq!("mxmxvkd,sqjhc,fvjkl".to_string(), solution.solve2());
    }
}
