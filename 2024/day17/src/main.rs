use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split("\n\n")
        .map(|l| {
            l.split(&['\n', ' ', ','])
                .filter_map(|sp| sp.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&input));
    println!("{:?}", solve2(&input));
}

fn combo_operand(o: usize, registers: &[usize]) -> usize {
    match o {
        0 | 1 | 2 | 3 => o,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}

fn solve1(input: &Vec<Vec<usize>>) -> String {
    let mut outputs = Vec::new();
    let mut registers = input[0].clone();
    let program = input[1].clone();

    let mut i = 0;
    while i < program.len() {
        match program[i] {
            0 => registers[0] /= 2_usize.pow(combo_operand(program[i + 1], &registers) as u32),
            1 => registers[1] ^= program[i + 1],
            2 => registers[1] = combo_operand(program[i + 1], &registers) % 8,
            3 if registers[0] == 0 => {}
            3 => {
                i = combo_operand(program[i + 1], &registers);
                continue;
            }
            4 => registers[1] ^= registers[2],
            5 => outputs.push(combo_operand(program[i + 1], &registers) % 8),
            6 => {
                registers[1] =
                    registers[0] / 2_usize.pow(combo_operand(program[i + 1], &registers) as u32)
            }
            7 => {
                registers[2] =
                    registers[0] / 2_usize.pow(combo_operand(program[i + 1], &registers) as u32)
            }
            _ => unreachable!(),
        }
        i += 2;
    }

    outputs
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve2(input: &Vec<Vec<usize>>) -> usize {
    let program = input[1].clone();

    let mut a = 8_usize.pow(15);
    for _ in 0.. {
        let mut registers = vec![a, 0, 0];
        let mut outputs = Vec::new();

        let mut i = 0;
        while i < program.len() {
            match program[i] {
                0 => registers[0] /= 2_usize.pow(combo_operand(program[i + 1], &registers) as u32),
                1 => registers[1] ^= program[i + 1],
                2 => registers[1] = combo_operand(program[i + 1], &registers) % 8,
                3 if registers[0] == 0 => {}
                3 => {
                    i = combo_operand(program[i + 1], &registers);
                    continue;
                }
                4 => registers[1] ^= registers[2],
                5 => outputs.push(combo_operand(program[i + 1], &registers) % 8),
                6 => {
                    registers[1] =
                        registers[0] / 2_usize.pow(combo_operand(program[i + 1], &registers) as u32)
                }
                7 => {
                    registers[2] =
                        registers[0] / 2_usize.pow(combo_operand(program[i + 1], &registers) as u32)
                }
                _ => unreachable!(),
            }
            i += 2;
        }

        let mut found = true;
        for i in (0..program.len()).rev() {
            if program[i] != outputs[i] {
                a += 8_usize.pow(i as u32 - 1).max(1);
                found = false;
                break;
            }
        }

        if found {
            return a;
        }
    }

    unreachable!()
}
