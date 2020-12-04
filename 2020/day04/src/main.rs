use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn solve(map: &Vec<HashMap<String, String>>) -> usize {
        let keys = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];
        map.iter()
            .filter(|m| keys.iter().all(|k| m.contains_key(k)))
            .count()
    }

    fn solve2(map: &Vec<HashMap<String, String>>) -> usize {
        map.iter()
            .filter(|m| {
                if let Some(v) = m.get(&"byr".to_string()) {
                    if v.as_str() < "1920" || "2002" < v.as_str() {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"iyr".to_string()) {
                    if v.as_str() < "2010" || "2020" < v.as_str() {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"eyr".to_string()) {
                    if v.as_str() < "2020" || "2030" < v.as_str() {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"hgt".to_string()) {
                    if v.ends_with("cm") {
                        let h = &v[..(v.len() - 2)];
                        if h < "150" || "193" < h {
                            return false;
                        }
                    } else if v.ends_with("in") {
                        let h = &v[..(v.len() - 2)];
                        if h < "59" || "76" < h {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"hcl".to_string()) {
                    if !v.starts_with("#")
                        || !v[1..]
                            .chars()
                            .all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"ecl".to_string()) {
                    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()) {
                        return false;
                    }
                } else {
                    return false;
                }
                if let Some(v) = m.get(&"pid".to_string()) {
                    if v.len() != 9 || !v.chars().all(|c| ('0' <= c && c <= '9')) {
                        return false;
                    }
                } else {
                    return false;
                }
                true
            })
            .count()
    }

    fn string_to_hashmap(input: String) -> Vec<HashMap<String, String>> {
        input
            .split("\n\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|l| {
                        let mut kv = l.split(':');
                        (
                            kv.next().unwrap().to_string(),
                            kv.next().unwrap().to_string(),
                        )
                    })
                    .collect::<HashMap<String, String>>()
            })
            .collect::<Vec<HashMap<String, String>>>()
    }
}

use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let input = Solution::string_to_hashmap(buf);
    println!("Part1: {}", Solution::solve(&input));
    println!("Part2: {}", Solution::solve2(&input));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day01() {
        let input_str = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        let input = Solution::string_to_hashmap(input_str.to_string());
        assert_eq!(2, Solution::solve(&input));

        let input_str = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
        let input = Solution::string_to_hashmap(input_str.to_string());
        assert_eq!(4, Solution::solve2(&input));
    }
}
