use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}
#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn simulate_register(instructions: &[Instruction]) -> Vec<i32> {
    let mut cycle = 1;
    let mut acc = vec![0; instructions.len() * 2 + 2];
    acc[1] = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
            }
            Instruction::Addx(value) => {
                cycle += 2;
                acc[cycle] += value;
            }
        }
    }

    for i in 2..acc.len() {
        acc[i] += acc[i - 1];
    }
    acc
}

fn solve1(instructions: &[Instruction]) -> i32 {
    let acc = simulate_register(instructions);
    (20..=220)
        .step_by(40)
        .map(|i| acc[i] * i as i32)
        .sum::<i32>()
}

fn solve2(instructions: &[Instruction]) -> String {
    let acc = simulate_register(instructions);
    let mut pixels = vec![Vec::with_capacity(40); 6];
    for row in 0..6 {
        for pos in 0..40 {
            pixels[row].push(if (acc[row * 40 + pos + 1] - pos as i32).abs() <= 1 {
                '#'
            } else {
                '.'
            });
        }
        pixels[row].push('\n');
    }
    pixels.into_iter().flatten().collect()
}

fn main() {
    let mut instructions = Vec::new();
    loop {
        let line: String = read();
        if line.is_empty() {
            break;
        }
        instructions.push(if line == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(
                line.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            )
        });
    }

    println!("{}", solve1(&instructions));
    println!("{}", solve2(&instructions));
}
