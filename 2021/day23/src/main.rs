use proconio::input;
use proconio::marker::Chars;

fn solve(rooms: &Vec<Vec<i32>>, n: usize) -> usize {
    let mut rooms = rooms.iter().cloned().collect();
    let mut hall = vec![-1; 11];
    for i in 0..4 {
        hall[room_position(i)] = -2;
    }

    let mut min_cost = std::usize::MAX;
    dfs(&mut rooms, &mut hall, 0, &mut min_cost, n);
    min_cost
}

fn dfs(
    rooms: &mut Vec<Vec<i32>>,
    hall: &mut Vec<i32>,
    cost: usize,
    min_cost: &mut usize,
    n: usize,
) {
    if is_organized(rooms, n) {
        *min_cost = std::cmp::min(*min_cost, cost);
        return;
    }

    // hall to room
    for i in 0..hall.len() {
        if hall[i] < 0 {
            continue;
        }
        let room = hall[i] as usize;
        let dest = room_position(room);
        if can_move(i, dest, hall) && rooms[room].iter().all(|x| x == &(room as i32)) {
            let moves = (i as i32 - dest as i32).abs() as usize + n - rooms[room].len();
            let new_cost = cost + calc_cost(room, moves);
            if &new_cost < min_cost {
                rooms[room].push(room as i32);
                hall[i] = -1;

                dfs(rooms, hall, new_cost, min_cost, n);

                hall[i] = rooms[room].pop().unwrap();
            }
        }
    }

    // room to hall
    for i in 0..4 {
        if rooms[i].iter().all(|x| x == &(i as i32)) {
            continue;
        }
        let src = room_position(i);
        for j in 0..hall.len() {
            if hall[j] == -2 || !can_move(src, j, hall) {
                continue;
            }
            let moves = (src as i32 - j as i32).abs() as usize + n + 1 - rooms[i].len();
            let new_cost = cost + calc_cost(*rooms[i].last().unwrap() as usize, moves);

            if &new_cost < min_cost {
                hall[j] = rooms[i].pop().unwrap();

                dfs(rooms, hall, new_cost, min_cost, n);

                rooms[i].push(hall[j]);
                hall[j] = -1;
            }
        }
    }
}

fn room_position(i: usize) -> usize {
    (i + 1) * 2
}

fn is_organized(rooms: &Vec<Vec<i32>>, n: usize) -> bool {
    (0..4).all(|i| rooms[i].len() == n && rooms[i].iter().all(|x| x == &(i as i32)))
}

fn can_move(from: usize, to: usize, hall: &Vec<i32>) -> bool {
    if from < to {
        ((from + 1)..=to).all(|i| hall[i] < 0)
    } else {
        (to..from).all(|i| hall[i] < 0)
    }
}

fn calc_cost(t: usize, moves: usize) -> usize {
    let costs = vec![1, 10, 100, 1000];
    moves * costs[t]
}

fn main() {
    input! {
        n: usize,
        start: [Chars; n],
    }

    let rooms = (0..4)
        .map(|i| {
            (0..n)
                .rev()
                .map(|j| start[j][i] as i32 - 0x41)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", solve(&rooms, n));
}
