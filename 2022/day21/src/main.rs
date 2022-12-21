use std::collections::HashMap;
use std::io::Read;

fn calculate(
    yell: &str,
    numbers: &mut HashMap<String, i64>,
    jobs: &HashMap<String, (String, String, char)>,
) -> i64 {
    if let Some(number) = numbers.get(yell) {
        *number
    } else {
        let (op1, op2, operator) = jobs.get(yell).unwrap();
        let number1 = calculate(op1, numbers, jobs);
        let number2 = calculate(op2, numbers, jobs);
        let number = match operator {
            &'+' => number1 + number2,
            &'-' => number1 - number2,
            &'*' => number1 * number2,
            &'/' => number1 / number2,
            _ => unreachable!(),
        };
        numbers.insert(yell.to_string(), number);
        number
    }
}

fn calculate_reverse(
    yell: &str,
    numbers: &mut HashMap<String, i64>,
    jobs: &HashMap<String, (String, String, char)>,
) -> i64 {
    if let Some(number) = numbers.get(yell) {
        *number
    } else {
        let (key, (op1, op2, operator)) = jobs
            .iter()
            .find(|(_, (op1, op2, _))| op1 == yell || op2 == yell)
            .unwrap();
        let number = if key == "root" {
            calculate(if op1 == yell { op2 } else { op1 }, numbers, jobs)
        } else if op1 == yell {
            let number1 = calculate_reverse(key, numbers, jobs);
            let number2 = calculate(op2, numbers, jobs);
            match operator {
                &'+' => number1 - number2,
                &'-' => number1 + number2,
                &'*' => number1 / number2,
                &'/' => number1 * number2,
                _ => unreachable!(),
            }
        } else {
            let number1 = calculate_reverse(key, numbers, jobs);
            let number2 = calculate(op1, numbers, jobs);
            match operator {
                &'+' => number1 - number2,
                &'-' => number2 - number1,
                &'*' => number1 / number2,
                &'/' => number2 / number1,
                _ => unreachable!(),
            }
        };
        numbers.insert(yell.to_string(), number);
        number
    }
}

fn solve1(numbers: &HashMap<String, i64>, jobs: &HashMap<String, (String, String, char)>) -> i64 {
    let mut numbers = numbers.clone();
    calculate("root", &mut numbers, jobs)
}

fn solve2(numbers: &HashMap<String, i64>, jobs: &HashMap<String, (String, String, char)>) -> i64 {
    let mut new_numbers = numbers.clone();
    new_numbers.remove("humn");

    calculate_reverse("humn", &mut new_numbers, &jobs)
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut numbers = HashMap::new();
    let mut jobs = HashMap::new();

    for line in buf.split('\n') {
        let yell = line[..4].to_string();
        if let Ok(number) = line[6..].parse::<i64>() {
            numbers.insert(yell, number);
        } else {
            let op1 = line[6..10].to_string();
            let op2 = line[13..17].to_string();
            let operator = line.as_bytes()[11] as char;
            jobs.insert(yell, (op1, op2, operator));
        }
    }

    println!("{}", solve1(&numbers, &jobs));
    println!("{}", solve2(&numbers, &jobs));
}
