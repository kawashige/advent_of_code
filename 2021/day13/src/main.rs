use proconio::input;

#[derive(Clone)]
enum Instruction {
    Y(usize),
    X(usize),
}

fn solve(paper: &Vec<Vec<bool>>, instructions: &Vec<Instruction>) -> usize {
    let mut paper = paper.clone();

    let (y_max, x_max) = simulate(&mut paper, &vec![instructions[0].clone()]);

    (0..=y_max)
        .map(|i| (0..=x_max).filter(|j| paper[i][*j]).count())
        .sum()
}

fn solve2(paper: &Vec<Vec<bool>>, instructions: &Vec<Instruction>) {
    let mut paper = paper.clone();

    let (y_max, x_max) = simulate(&mut paper, instructions);

    for i in 0..=y_max {
        println!(
            "{}",
            paper[i][..=x_max]
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn simulate(paper: &mut Vec<Vec<bool>>, instructions: &Vec<Instruction>) -> (usize, usize) {
    let mut y_max = paper.len() - 1;
    let mut x_max = paper[0].len() - 1;

    for instruction in instructions {
        match instruction {
            Instruction::Y(y) => {
                (0..*y).for_each(|i| {
                    (0..=x_max).for_each(|j| {
                        paper[i][j] |= paper[y_max - i][j];
                    });
                });
                y_max = y - 1;
            }
            Instruction::X(x) => {
                (0..*x).for_each(|j| {
                    (0..=y_max).for_each(|i| {
                        paper[i][j] |= paper[i][x_max - j];
                    });
                });
                x_max = x - 1;
            }
        }
    }

    (y_max, x_max)
}

fn main() {
    input! {
        n: usize,
        dots: [String; n],
        m: usize,
        instructions: [[String; 3]; m]
    }

    let dots = dots
        .into_iter()
        .map(|d| {
            let coodinate = d
                .split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (coodinate[0], coodinate[1])
        })
        .collect::<Vec<_>>();

    let r = dots.iter().map(|(x, _)| x).max().unwrap();
    let c = dots.iter().map(|(_, y)| y).max().unwrap();
    let mut paper = vec![vec![false; r + 1]; c + 1];
    for (x, y) in dots {
        paper[y][x] = true;
    }

    let instructions = instructions
        .into_iter()
        .map(|i| {
            if i[2].starts_with("y=") {
                Instruction::Y(i[2].strip_prefix("y=").unwrap().parse::<usize>().unwrap())
            } else {
                Instruction::X(i[2].strip_prefix("x=").unwrap().parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&paper, &instructions));
    println!("part2:");
    solve2(&paper, &instructions);
}
