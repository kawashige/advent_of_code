use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (values, connections) = {
        let mut sp = buf.split("\n\n");
        let values = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|l| {
                let mut sp = l.split(": ");
                let key = sp.next().unwrap().to_string();
                let value = sp.next().unwrap().parse::<usize>().unwrap();
                (key, value)
            })
            .collect::<HashMap<_, _>>();

        let connections = sp
            .next()
            .unwrap()
            .split("\n")
            .map(|l| {
                l.replace("-> ", "")
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (values, connections)
    };

    println!("{:?}", solve1(&values, &connections));
    println!("{:?}", solve2(&values, &connections));
}

fn solve1(values: &HashMap<String, usize>, connections: &Vec<Vec<String>>) -> usize {
    let mut values = values.clone();
    let mut changed = true;

    while changed {
        changed = false;

        for c in connections {
            if values.contains_key(&c[0])
                && values.contains_key(&c[2])
                && !values.contains_key(&c[3])
            {
                changed = true;
                let v1 = values[&c[0]];
                let v2 = values[&c[2]];
                let value = match c[1].as_str() {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "XOR" => v1 ^ v2,
                    _ => unreachable!(),
                };
                values.insert(c[3].clone(), value);
            }
        }
    }

    let mut result = 0;
    for (k, v) in values {
        if !k.starts_with("z") || v == 0 {
            continue;
        }
        let i = k[1..].parse::<usize>().unwrap();
        result |= 1 << i;
    }
    result
}

fn to_digit(t: &str, values: &HashMap<String, usize>) -> usize {
    let mut d = 0;
    for (k, v) in values {
        if !k.starts_with(t) || v == &0 {
            continue;
        }
        let i = k[1..].parse::<usize>().unwrap();
        d |= 1 << i;
    }
    d
}

fn path(t: &str, connections: &Vec<Vec<String>>, v: &mut Vec<String>) {
    v.push(t.to_string());
    if t.starts_with("x") || t.starts_with("y") {
        return;
    }
    for i in 0..connections.len() {
        if connections[i][3] == t {
            path(&connections[i][0], connections, v);
            path(&connections[i][2], connections, v);
        }
    }
}

fn solve2(values: &HashMap<String, usize>, connections: &Vec<Vec<String>>) -> String {
    let mut z_without_xor = Vec::new();
    let mut xor_without_z = Vec::new();
    for i in 0..connections.len() {
        if connections[i][3].starts_with("z")
            && connections[i][1] != "XOR"
            && connections[i][3] != "z45"
        {
            z_without_xor.push((connections[i][3].clone(), i));
        }
        if !connections[i][0].starts_with("x")
            && !connections[i][0].starts_with("y")
            && connections[i][1] == "XOR"
            && !connections[i][3].starts_with("z")
        {
            xor_without_z.push((connections[i][3].clone(), i));
        }
    }

    let mut pairs = Vec::new();
    for i in 0..xor_without_z.len() {
        let mut v = Vec::new();
        path(&xor_without_z[i].0, connections, &mut v);
        let max_input = v
            .into_iter()
            .filter_map(|p| p[1..].parse::<usize>().ok())
            .max()
            .unwrap();
        for j in 0..z_without_xor.len() {
            if z_without_xor[j].0[1..].parse::<usize>().unwrap() == max_input {
                pairs.push((xor_without_z[i].clone(), z_without_xor[j].clone()));
                break;
            }
        }
    }

    let mut conn = connections.clone();
    for i in 0..pairs.len() {
        conn[pairs[i].0 .1][3] = pairs[i].1 .0.clone();
        conn[pairs[i].1 .1][3] = pairs[i].0 .0.clone();
    }
    let x = to_digit("x", &values);
    let y = to_digit("y", &values);

    for i in 0..connections.len() {
        for j in 0..i {
            if (0..pairs.len()).any(|k| {
                pairs[k].0 .1 == i || pairs[k].1 .1 == i || pairs[k].0 .1 == j || pairs[k].1 .1 == j
            }) {
                continue;
            }
            let mut values_after = values.clone();
            let mut c = conn.clone();
            let tmp = c[i][3].clone();
            c[i][3] = c[j][3].clone();
            c[j][3] = tmp;

            let mut changed = true;

            while changed {
                changed = false;

                for c in &c {
                    if values_after.contains_key(&c[0])
                        && values_after.contains_key(&c[2])
                        && !values_after.contains_key(&c[3])
                    {
                        changed = true;
                        let v1 = values_after[&c[0]];
                        let v2 = values_after[&c[2]];
                        let value = match c[1].as_str() {
                            "AND" => v1 & v2,
                            "OR" => v1 | v2,
                            "XOR" => v1 ^ v2,
                            _ => unreachable!(),
                        };
                        values_after.insert(c[3].clone(), value);
                    }
                }
            }

            let z = to_digit("z", &values_after);
            if z == x + y && check(values, &c) {
                let mut result = pairs
                    .iter()
                    .map(|p| vec![p.0 .0.to_string(), p.1 .0.to_string()])
                    .flatten()
                    .collect::<Vec<_>>();
                result.push(connections[i][3].to_string());
                result.push(connections[j][3].to_string());
                result.sort_unstable();
                return result.join(",");
            }
        }
    }
    unreachable!()
}

fn check(values: &HashMap<String, usize>, connections: &Vec<Vec<String>>) -> bool {
    let mut values0 = values.clone();
    let mut values1 = values.clone();
    for (k, _) in values {
        if k.starts_with("y") {
            values0.insert(k.to_string(), 0);
            values1.insert(k.to_string(), 1);
        }
    }
    let x0 = to_digit("x", &values0);
    let x1 = to_digit("x", &values1);
    let y0 = to_digit("y", &values0);
    let y1 = to_digit("y", &values1);

    let mut changed = true;
    while changed {
        changed = false;

        for c in connections {
            if values0.contains_key(&c[0])
                && values0.contains_key(&c[2])
                && !values0.contains_key(&c[3])
            {
                changed = true;
                let v1 = values0[&c[0]];
                let v2 = values0[&c[2]];
                let value = match c[1].as_str() {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "XOR" => v1 ^ v2,
                    _ => unreachable!(),
                };
                values0.insert(c[3].clone(), value);
            }
        }
    }
    let z0 = to_digit("z", &values0);
    if z0 != x0 + y0 {
        return false;
    }

    let mut changed = true;
    while changed {
        changed = false;

        for c in connections {
            if values1.contains_key(&c[0])
                && values1.contains_key(&c[2])
                && !values1.contains_key(&c[3])
            {
                changed = true;
                let v1 = values1[&c[0]];
                let v2 = values1[&c[2]];
                let value = match c[1].as_str() {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "XOR" => v1 ^ v2,
                    _ => unreachable!(),
                };
                values1.insert(c[3].clone(), value);
            }
        }
    }
    let z1 = to_digit("z", &values1);
    if z1 != x1 + y1 {
        return false;
    }
    true
}
