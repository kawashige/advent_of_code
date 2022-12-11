use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Pow,
}

fn solve1(
    items: &Vec<Vec<usize>>,
    operations: &[Operation],
    tests: &[(usize, [usize; 2])],
) -> usize {
    let mut inspections = vec![0; items.len()];
    let mut items = items.clone();

    for _ in 0..20 {
        for i in 0..items.len() {
            inspections[i] += items[i].len();
            while let Some(old) = items[i].pop() {
                let mut new = match operations[i] {
                    Operation::Add(x) => old + x,
                    Operation::Multiply(x) => old * x,
                    Operation::Pow => old * old,
                };
                new /= 3;
                items[tests[i].1[if new % tests[i].0 == 0 { 0 } else { 1 }]].push(new);
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(&a));
    inspections[0] * inspections[1]
}

fn solve2(
    items: &Vec<Vec<usize>>,
    operations: &[Operation],
    tests: &[(usize, [usize; 2])],
) -> usize {
    let mut inspections = vec![0; items.len()];
    let mut items = items
        .iter()
        .map(|item| {
            item.iter()
                .map(|level| {
                    tests
                        .iter()
                        .map(|(divisor, _)| level % divisor)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for _ in 0..10000 {
        for i in 0..items.len() {
            inspections[i] += items[i].len();
            while let Some(old) = items[i].pop() {
                let new = (0..old.len())
                    .map(|j| {
                        (match operations[i] {
                            Operation::Add(x) => old[j] + x,
                            Operation::Multiply(x) => old[j] * x,
                            Operation::Pow => old[j] * old[j],
                        }) % tests[j].0
                    })
                    .collect::<Vec<_>>();
                items[tests[i].1[if new[i] == 0 { 0 } else { 1 }]].push(new);
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(&a));
    inspections[0] * inspections[1]
}

fn main() {
    let mut items = Vec::new();
    let mut operations = Vec::new();
    let mut tests = Vec::new();
    loop {
        let line: String = read();
        if !line.starts_with("Monkey ") {
            break;
        }
        items.push(
            read::<String>()
                .trim_start_matches("Starting items: ")
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );

        let line = read::<String>();
        let operation = line.trim_start_matches("Operation: new = old ");
        operations.push(if operation == "* old" {
            Operation::Pow
        } else if operation.starts_with("*") {
            Operation::Multiply(operation[2..].parse().unwrap())
        } else {
            Operation::Add(operation[2..].parse().unwrap())
        });

        let test = read::<String>()[19..].parse::<usize>().unwrap();
        let throw_true = read::<String>()[25..].parse::<usize>().unwrap();
        let throw_false = read::<String>()[26..].parse::<usize>().unwrap();
        tests.push((test, [throw_true, throw_false]));

        read::<String>();
    }

    println!("{}", solve1(&items, &operations, &tests));
    println!("{}", solve2(&items, &operations, &tests));
}
