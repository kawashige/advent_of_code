use std::io::Read;
use z3::ast;
use z3::ast::Ast;
use z3::{self, SatResult};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let machines = buf
        .split("\n\n")
        .map(|m| {
            m.split("\n")
                .map(|l| {
                    l.split(&['+', ',', '=', ' '])
                        .filter_map(|d| d.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve1(&machines));
    println!("{:?}", solve2(&machines));
}

fn solve1(machines: &Vec<Vec<Vec<usize>>>) -> usize {
    machines
        .iter()
        .map(|m| {
            let mut cost = std::usize::MAX;
            for i in 0..100 {
                for j in 0..100 {
                    if m[0][0] * i + m[1][0] * j == m[2][0] && m[0][1] * i + m[1][1] * j == m[2][1]
                    {
                        cost = cost.min(i * 3 + j);
                    }
                }
            }
            if cost == std::usize::MAX {
                0
            } else {
                cost
            }
        })
        .sum()
}

fn solve2(machines: &Vec<Vec<Vec<usize>>>) -> usize {
    const P: usize = 10000000000000;
    machines
        .iter()
        .map(|m| {
            let t = [m[2][0] + P, m[2][1] + P];
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);

            let x = ast::Int::new_const(&ctx, "x");
            let y = ast::Int::new_const(&ctx, "y");

            let x_a = ast::Int::from_i64(&ctx, m[0][0] as i64);
            let y_a = ast::Int::from_i64(&ctx, m[0][1] as i64);
            let x_b = ast::Int::from_i64(&ctx, m[1][0] as i64);
            let y_b = ast::Int::from_i64(&ctx, m[1][1] as i64);
            let t_x = ast::Int::from_i64(&ctx, t[0] as i64);
            let t_y = ast::Int::from_i64(&ctx, t[1] as i64);
            let a = ast::Int::from_i64(&ctx, 3);

            let solver = z3::Optimize::new(&ctx);
            solver.minimize(&ast::Int::add(&ctx, &[&ast::Int::mul(&ctx, &[&a, &x]), &y]));

            solver.assert(
                &ast::Int::add(
                    &ctx,
                    &[
                        &ast::Int::mul(&ctx, &[&x_a, &x]),
                        &ast::Int::mul(&ctx, &[&x_b, &y]),
                    ],
                )
                ._eq(&t_x),
            );
            solver.assert(
                &ast::Int::add(
                    &ctx,
                    &[
                        &ast::Int::mul(&ctx, &[&y_a, &x]),
                        &ast::Int::mul(&ctx, &[&y_b, &y]),
                    ],
                )
                ._eq(&t_y),
            );

            (match solver.check(&[]) {
                SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let x_value = model.eval(&x, true).unwrap().as_i64().unwrap();
                    let y_value = model.eval(&y, true).unwrap().as_i64().unwrap();
                    x_value * 3 + y_value
                }
                _ => 0,
            }) as usize
        })
        .sum()
}
