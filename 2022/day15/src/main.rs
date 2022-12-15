use std::{collections::HashSet, io::Read};

fn manhattan_distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn compact(ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    for i in 0..ranges.len() {
        if 0 < i && ranges[i].0 - 1 <= result.last().unwrap().1 {
            let prev = result.pop().unwrap();
            result.push((ranges[i].0.min(prev.0), ranges[i].1.max(prev.1)));
        } else {
            result.push(ranges[i]);
        }
    }
    result
}

fn cannnot_contains_beacon_ranges(
    sensors: &Vec<((i32, i32), (i32, i32))>,
    target_y: i32,
) -> Vec<(i32, i32)> {
    let mut ranges = sensors
        .iter()
        .filter_map(|(sensor, beacon)| {
            let dist = manhattan_distance(sensor, beacon);
            if dist <= (target_y - sensor.1).abs() {
                None
            } else {
                let count = dist - (target_y - sensor.1).abs();
                Some((sensor.0 - count, sensor.0 + count))
            }
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable();
    compact(ranges)
}

fn solve1(sensors: &Vec<((i32, i32), (i32, i32))>, target_y: i32) -> i32 {
    let ranges = cannnot_contains_beacon_ranges(sensors, target_y);
    let beacons = sensors
        .iter()
        .filter_map(|(_, beacon)| {
            if beacon.1 == target_y {
                Some(beacon)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    ranges
        .into_iter()
        .map(|range| {
            let beacons = beacons
                .iter()
                .filter(|beacon| (range.0..=range.1).contains(&beacon.1))
                .count();
            range.1 - range.0 + 1 - beacons as i32
        })
        .sum::<i32>()
}

fn solve2(sensors: &Vec<((i32, i32), (i32, i32))>, max_coodinate: i32) -> usize {
    for y in 0..max_coodinate {
        let ranges = cannnot_contains_beacon_ranges(sensors, y);
        if 1 < ranges.len() {
            return (ranges[1].0 - 1) as usize * 4000000 + y as usize;
        }
    }

    unreachable!()
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.split('\n').collect::<Vec<_>>();
    let target_y = lines[0].parse::<i32>().unwrap();
    let sensors = lines[1..]
        .iter()
        .map(|line| {
            let values = line
                .split(&['=', ',', ':'])
                .filter_map(|sp| sp.parse::<i32>().ok())
                .collect::<Vec<_>>();
            ((values[0], values[1]), (values[2], values[3]))
        })
        .collect::<Vec<_>>();

    println!("{}", solve1(&sensors, target_y));
    println!("{}", solve2(&sensors, target_y * 2));
}
