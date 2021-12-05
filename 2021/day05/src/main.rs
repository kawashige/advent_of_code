use proconio::input;

fn solve(lines: &Vec<Vec<Vec<usize>>>) -> usize {
    let filtered = lines
        .into_iter()
        .filter(|v| v[0][0] == v[1][0] || v[0][1] == v[1][1])
        .cloned()
        .collect::<Vec<_>>();

    calc(&filtered)
}

fn solve2(lines: &Vec<Vec<Vec<usize>>>) -> usize {
    calc(lines)
}

fn calc(lines: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut diagram = vec![vec![0; 1000]; 1000];

    for v in lines {
        if v[0][0] == v[1][0] {
            (v[0][1]..=v[1][1]).for_each(|j| diagram[v[0][0]][j] += 1);
        } else if v[0][1] == v[1][1] {
            (v[0][0]..=v[1][0]).for_each(|j| diagram[j][v[0][1]] += 1);
        } else if v[0][1] < v[1][1] {
            (v[0][0]..=v[1][0]).for_each(|i| diagram[i][v[0][1] + i - v[0][0]] += 1);
        } else {
            (v[0][0]..=v[1][0]).for_each(|i| diagram[i][v[0][1] + v[0][0] - i] += 1);
        }
    }

    diagram
        .into_iter()
        .map(|row| row.into_iter().filter(|x| &1 < x).count())
        .sum::<usize>()
}

fn main() {
    input! {
        n: usize,
        lines: [(String, String, String); n]
    }

    let lines = lines
        .into_iter()
        .map(|(start, _, end)| {
            let mut v = vec![
                start
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                end.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            ];
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>();

    println!("part1: {}", solve(&lines));
    println!("part2: {}", solve2(&lines));
}
