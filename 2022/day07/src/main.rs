use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn calculate_directory_size(dir: &str, files: &HashMap<String, usize>) -> usize {
    files
        .iter()
        .filter(|(path, _)| path.starts_with(dir))
        .map(|(_, size)| size)
        .sum::<usize>()
}

fn solve1(dirs: &HashSet<String>, files: &HashMap<String, usize>) -> usize {
    dirs.iter()
        .map(|dir| calculate_directory_size(dir, files))
        .filter(|size| size <= &100_000)
        .sum()
}

fn solve2(dirs: &HashSet<String>, files: &HashMap<String, usize>) -> usize {
    let used = files.values().sum::<usize>();
    let mut sizes = dirs
        .iter()
        .map(|dir| calculate_directory_size(dir, files))
        .collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes
        .into_iter()
        .find(|size| 30_000_000 <= 70_000_000 - used + size)
        .unwrap()
}

fn main() {
    let mut files = HashMap::new();
    let mut dirs = HashSet::new();
    let mut paths = vec![];

    loop {
        let line: String = read();
        if line.is_empty() {
            break;
        }
        let sp = line.split_ascii_whitespace().collect::<Vec<_>>();
        if sp[0] == "$" {
            if sp[1] == "cd" {
                match sp[2] {
                    ".." => {
                        paths.pop().unwrap();
                    }
                    "/" => paths.clear(),
                    _ => {
                        paths.push(sp[2].to_string());
                        dirs.insert(paths.join("/"));
                    }
                }
            }
        } else if sp[0] != "dir" {
            files.insert(
                format!("{}/{}", paths.join("/"), sp[1]),
                sp[0].parse::<usize>().unwrap(),
            );
        }
    }

    println!("{}", solve1(&dirs, &files));
    println!("{}", solve2(&dirs, &files));
}
