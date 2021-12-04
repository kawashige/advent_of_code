use proconio::input;

fn solve(numbers: &[usize], boards: &Vec<Vec<Vec<usize>>>) -> usize {
    simulate(numbers, boards)
        .into_iter()
        .min_by_key(|(i, _)| *i)
        .unwrap()
        .1
}

fn solve2(numbers: &[usize], boards: &Vec<Vec<Vec<usize>>>) -> usize {
    simulate(numbers, boards)
        .into_iter()
        .max_by_key(|(i, _)| *i)
        .unwrap()
        .1
}

fn simulate(numbers: &[usize], boards: &Vec<Vec<Vec<usize>>>) -> Vec<(usize, usize)> {
    let number_index = numbers
        .iter()
        .enumerate()
        .fold([100; 100], |mut index, (i, x)| {
            index[*x] = i;
            index
        });

    boards
        .into_iter()
        .map(|board| {
            let min_index = (0..5)
                .map(|i| {
                    (0..5)
                        .map(|j| number_index[board[i][j]])
                        .max()
                        .unwrap()
                        .min((0..5).map(|j| number_index[board[j][i]]).max().unwrap())
                })
                .min()
                .unwrap();

            let score = board
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|x| min_index < number_index[**x])
                        .sum::<usize>()
                })
                .sum::<usize>();

            (min_index, score * numbers[min_index])
        })
        .collect()
}

fn main() {
    input! {
        n: usize,
        numbers: String,
        boards: [[[usize; 5]; 5]; n]
    }

    let numbers = numbers
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&numbers, &boards));
    println!("part2: {}", solve2(&numbers, &boards));
}
