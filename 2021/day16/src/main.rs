use proconio::input;
use proconio::marker::Chars;

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(version: usize, type_id: usize, value: usize, sub_packets: Vec<Packet>) -> Self {
        Self {
            version,
            type_id,
            value,
            sub_packets,
        }
    }
}

fn solve(hex: &str) -> usize {
    let packets = parse_hex(hex);
    packets.iter().map(|packet| sum_version(packet)).sum()
}

fn solve2(hex: &str) -> usize {
    let packets = parse_hex(hex);
    packets[0].value
}

fn parse_hex(hex: &str) -> Vec<Packet> {
    let mut i = 0;
    let mut packets = Vec::new();
    while i < hex.len() {
        let (j, packet) = parse_paket(hex, i, true);
        i = j;
        packets.push(packet);
    }
    packets
}

fn sum_version(packet: &Packet) -> usize {
    packet.version
        + packet
            .sub_packets
            .iter()
            .map(|s| sum_version(s))
            .sum::<usize>()
}

fn parse_paket(hex: &str, start: usize, padding: bool) -> (usize, Packet) {
    let mut i = start;

    let version = usize::from_str_radix(&hex[i..(i + 3)], 2).unwrap();
    i += 3;
    let type_id = usize::from_str_radix(&hex[i..(i + 3)], 2).unwrap();
    i += 3;

    let mut sub_packets = Vec::new();
    let value: usize;

    if type_id == 4 {
        let mut literal_value = String::new();
        let mut is_last = false;
        while !is_last {
            is_last = &hex[i..(i + 1)] == "0";
            i += 1;
            literal_value += &hex[i..(i + 4)];
            i += 4;
        }
        value = usize::from_str_radix(&literal_value, 2).unwrap();
    } else {
        let length_type_id = usize::from_str_radix(&hex[i..(i + 1)], 2).unwrap();
        i += 1;
        if length_type_id == 0 {
            let length_of_bits = usize::from_str_radix(&hex[i..(i + 15)], 2).unwrap();
            i += 15;
            let subpacket_start_i = i;
            while i - subpacket_start_i < length_of_bits {
                let (j, sub_packet) = parse_paket(hex, i, false);
                i = j;
                sub_packets.push(sub_packet);
            }
        } else {
            let num_of_puckets = usize::from_str_radix(&hex[i..(i + 11)], 2).unwrap();
            i += 11;
            for _ in 0..num_of_puckets {
                let (j, sub_packet) = parse_paket(hex, i, false);
                i = j;
                sub_packets.push(sub_packet);
            }
        }

        value = calc_value(type_id, &sub_packets);
    }

    if padding {
        while (i - start) % 8 != 0 {
            i += 1;
        }
    }

    (i, Packet::new(version, type_id, value, sub_packets))
}

fn calc_value(type_id: usize, packets: &Vec<Packet>) -> usize {
    match type_id {
        0 => packets.iter().map(|p| p.value).sum(),
        1 => packets.iter().map(|p| p.value).product(),
        2 => packets.iter().map(|p| p.value).min().unwrap(),
        3 => packets.iter().map(|p| p.value).max().unwrap(),
        5 => {
            if packets[0].value > packets[1].value {
                1
            } else {
                0
            }
        }
        6 => {
            if packets[0].value < packets[1].value {
                1
            } else {
                0
            }
        }
        7 => {
            if packets[0].value == packets[1].value {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        transmission: Chars
    }

    let hex = transmission
        .into_iter()
        .map(|c| {
            format!(
                "{:0>4}",
                format!("{:b}", usize::from_str_radix(&c.to_string(), 16).unwrap())
            )
        })
        .collect::<String>();

    println!("part1: {}", solve(&hex));
    println!("part2: {}", solve2(&hex));
}
