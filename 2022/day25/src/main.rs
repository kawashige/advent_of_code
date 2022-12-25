use proconio::input;

fn solve1(snafu_numbers: &[String]) -> String {
    let max = snafu_numbers.iter().map(|s| s.len()).max().unwrap();
    let mut counts = snafu_numbers.iter().fold(vec![0; max], |mut counts, s| {
        for i in 0..s.len() {
            counts[s.len() - 1 - i] += match s.as_bytes()[i] {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => unreachable!(),
            };
        }
        counts
    });

    for i in 0..counts.len() {
        while 5 <= counts[i] {
            counts[i] -= 5;
            if i == counts.len() - 1 {
                counts.push(0);
            }
            counts[i + 1] += 1;
        }
        while counts[i] <= -5 {
            counts[i] += 5;
            if i == counts.len() - 1 {
                counts.push(0);
            }
            counts[i + 1] -= 1;
        }

        match counts[i] {
            4 => {
                counts[i] = -1;
                counts[i + 1] += 1;
            }
            3 => {
                counts[i] = -2;
                counts[i + 1] += 1;
            }
            -3 => {
                counts[i] = 2;
                counts[i + 1] -= 1;
            }
            -4 => {
                counts[i] = 1;
                counts[i + 1] -= 1;
            }
            _ => {}
        }
    }

    counts
        .into_iter()
        .rev()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    input! {
        n: usize,
        snafu_numbers: [String; n]
    }

    println!("{}", solve1(&snafu_numbers));
}
