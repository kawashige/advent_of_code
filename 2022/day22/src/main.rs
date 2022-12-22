use std::{collections::VecDeque, io::Read};

#[derive(Debug)]
enum Path {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn move_position(
    position: (usize, usize),
    direction: usize,
    board: &Vec<Vec<char>>,
) -> (usize, usize) {
    let move_position = match direction {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let mut next_position = (
        (board.len() as i32 + position.0 as i32 + move_position.0) as usize % board.len(),
        (board[position.0].len() as i32 + position.1 as i32 + move_position.1) as usize
            % board[position.0].len(),
    );
    while board[next_position.0][next_position.1] == ' ' {
        next_position = (
            (board.len() as i32 + next_position.0 as i32 + move_position.0) as usize % board.len(),
            (board[next_position.0].len() as i32 + next_position.1 as i32 + move_position.1)
                as usize
                % board[next_position.0].len(),
        );
    }
    next_position
}

fn move_cube_position(
    position: (usize, usize),
    direction: usize,
    board: &Vec<Vec<char>>,
    squares: &Vec<(i32, i32)>,
    vertex_index: &Vec<Vec<usize>>,
    square_size: usize,
) -> ((usize, usize), usize) {
    let move_position = match direction {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    if (direction == 0
        && (position.1 == board[0].len() - 1 || board[position.0][position.1 + 1] == ' '))
        || (direction == 1
            && (position.0 == board.len() - 1 || board[position.0 + 1][position.1] == ' '))
        || (direction == 2 && (position.1 == 0 || board[position.0][position.1 - 1] == ' '))
        || (direction == 3 && (position.0 == 0 || board[position.0 - 1][position.1] == ' '))
    {
        let current_square = squares
            .iter()
            .position(|square| {
                square.0 == (position.0 / square_size) as i32
                    && square.1 == (position.1 / square_size) as i32
            })
            .unwrap();
        let edge = match direction {
            0 => (1, 2),
            1 => (2, 3),
            2 => (3, 0),
            3 => (0, 1),
            _ => unreachable!(),
        };
        let target_suqare = (0..vertex_index.len())
            .find(|i| {
                i != &current_square
                    && vertex_index[*i].contains(&vertex_index[current_square][edge.0])
                    && vertex_index[*i].contains(&vertex_index[current_square][edge.1])
            })
            .unwrap();
        let target_edge = (
            (0..4)
                .find(|i| vertex_index[target_suqare][*i] == vertex_index[current_square][edge.0])
                .unwrap(),
            (0..4)
                .find(|i| vertex_index[target_suqare][*i] == vertex_index[current_square][edge.1])
                .unwrap(),
        );
        let offset = match edge {
            (0, 1) => position.1 as i32 - squares[current_square].1 * square_size as i32,
            (1, 2) => position.0 as i32 - squares[current_square].0 * square_size as i32,
            (2, 3) => {
                squares[current_square].1 * square_size as i32 + square_size as i32
                    - 1
                    - position.1 as i32
            }
            (3, 0) => {
                squares[current_square].0 * square_size as i32 + square_size as i32
                    - 1
                    - position.0 as i32
            }
            _ => unreachable!(),
        } as usize;
        let next_position = (
            squares[target_suqare].0 as usize * square_size
                + if 1 < target_edge.0 {
                    square_size - 1
                } else {
                    0
                },
            squares[target_suqare].1 as usize * square_size
                + if target_edge.0 == 1 || target_edge.0 == 2 {
                    square_size - 1
                } else {
                    0
                },
        );
        match target_edge {
            (0, 1) => ((next_position.0, next_position.1 + offset), 1),
            (0, 3) => ((next_position.0 + offset, next_position.1), 0),
            (1, 0) => ((next_position.0, next_position.1 - offset), 1),
            (1, 2) => ((next_position.0 + offset, next_position.1), 2),
            (2, 1) => ((next_position.0 - offset, next_position.1), 2),
            (2, 3) => ((next_position.0, next_position.1 - offset), 3),
            (3, 2) => ((next_position.0, next_position.1 + offset), 3),
            (3, 0) => ((next_position.0 - offset, next_position.1), 0),
            _ => unreachable!(),
        }
    } else {
        let next_position = (
            (position.0 as i32 + move_position.0) as usize,
            (position.1 as i32 + move_position.1) as usize,
        );
        (next_position, direction)
    }
}
fn solve1(board: &Vec<Vec<char>>, path: &Vec<Path>) -> usize {
    let mut position = (0, board[0].iter().position(|c| c == &'.').unwrap());
    let mut direction = 0;

    for p in path {
        match p {
            Path::TurnLeft => direction = (direction + 3) % 4,
            Path::TurnRight => direction = (direction + 1) % 4,
            Path::Move(tiles) => {
                for _ in 0..*tiles {
                    let next_position = move_position(position, direction, board);
                    if board[next_position.0][next_position.1] == '#' {
                        break;
                    }
                    position = next_position;
                }
            }
        }
    }

    1000 * (position.0 + 1) + 4 * (position.1 + 1) + direction
}

fn find_vertex_index(index: usize, vertex_index: &Vec<usize>) -> usize {
    let connected = [
        [1, 3, 4],
        [0, 2, 5],
        [1, 3, 6],
        [0, 2, 7],
        [0, 5, 7],
        [1, 4, 6],
        [2, 5, 7],
        [3, 4, 6],
    ];
    *connected[index]
        .iter()
        .find(|i| !vertex_index.contains(i))
        .unwrap()
}

fn find_adjacent_square(i: usize, squares: &[(i32, i32)]) -> Vec<usize> {
    (0..squares.len())
        .filter(|j| {
            j != &i
                && (squares[i].0 - squares[*j].0).abs() + (squares[i].1 - squares[*j].1).abs() == 1
        })
        .collect()
}

fn get_squares(board: &Vec<Vec<char>>, square_size: usize) -> Vec<(i32, i32)> {
    let mut squares = vec![];
    for i in 0..board.len() / square_size {
        for j in 0..board[0].len() / square_size {
            if board[i * square_size][j * square_size] != ' ' {
                squares.push((i as i32, j as i32));
            }
        }
    }
    squares
}

fn assign_vertex_index(squares: &Vec<(i32, i32)>) -> Vec<Vec<usize>> {
    let mut seen = vec![false; squares.len()];
    let mut vertex_index = vec![vec![]; squares.len()];
    let mut queue = VecDeque::new();
    vertex_index[0] = vec![0, 1, 2, 3];
    seen[0] = true;
    for adjacent in find_adjacent_square(0, &squares) {
        queue.push_back(adjacent);
    }

    while let Some(square) = queue.pop_front() {
        if seen[square] {
            continue;
        }
        seen[square] = true;
        for adjacent in find_adjacent_square(square, &squares) {
            if !seen[adjacent] {
                queue.push_back(adjacent);
                continue;
            }
            let vertex = match (
                squares[square].0 - squares[adjacent].0,
                squares[square].1 - squares[adjacent].1,
            ) {
                (1, 0) => vec![
                    vertex_index[adjacent][3],
                    vertex_index[adjacent][2],
                    find_vertex_index(vertex_index[adjacent][2], &vertex_index[adjacent]),
                    find_vertex_index(vertex_index[adjacent][3], &vertex_index[adjacent]),
                ],
                (0, 1) => vec![
                    vertex_index[adjacent][1],
                    find_vertex_index(vertex_index[adjacent][1], &vertex_index[adjacent]),
                    find_vertex_index(vertex_index[adjacent][2], &vertex_index[adjacent]),
                    vertex_index[adjacent][2],
                ],
                (0, -1) => vec![
                    find_vertex_index(vertex_index[adjacent][0], &vertex_index[adjacent]),
                    vertex_index[adjacent][0],
                    vertex_index[adjacent][3],
                    find_vertex_index(vertex_index[adjacent][3], &vertex_index[adjacent]),
                ],
                (-1, 0) => vec![
                    find_vertex_index(vertex_index[adjacent][0], &vertex_index[adjacent]),
                    find_vertex_index(vertex_index[adjacent][1], &vertex_index[adjacent]),
                    vertex_index[adjacent][1],
                    vertex_index[adjacent][0],
                ],
                _ => unreachable!(),
            };
            vertex_index[square] = vertex;
        }
    }
    vertex_index
}

fn solve2(board: &Vec<Vec<char>>, path: &Vec<Path>, square_size: usize) -> usize {
    let squares = get_squares(&board, square_size);
    let vertex_index = assign_vertex_index(&squares);

    let mut position = (0, board[0].iter().position(|c| c == &'.').unwrap());
    let mut direction = 0;

    for p in path {
        match p {
            Path::TurnLeft => direction = (direction + 3) % 4,
            Path::TurnRight => direction = (direction + 1) % 4,
            Path::Move(tiles) => {
                for _ in 0..*tiles {
                    let (next_position, next_direction) = move_cube_position(
                        position,
                        direction,
                        board,
                        &squares,
                        &vertex_index,
                        square_size,
                    );
                    if board[next_position.0][next_position.1] == '#' {
                        break;
                    }
                    position = next_position;
                    direction = next_direction;
                }
            }
        }
    }

    1000 * (position.0 + 1) + 4 * (position.1 + 1) + direction
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (board, path, square_size) = {
        let sp = buf.split("\n\n").collect::<Vec<_>>();
        let max_column = sp[0].split('\n').map(|line| line.len()).max().unwrap();
        let board = sp[0]
            .split('\n')
            .map(|line| {
                line.chars()
                    .chain(std::iter::repeat(' ').take(max_column - line.len()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut path = Vec::new();
        let mut num = 0;
        for c in sp[1].chars() {
            match c {
                'R' | 'L' => {
                    if num != 0 {
                        path.push(Path::Move(num));
                    }
                    num = 0;
                    path.push(if c == 'R' {
                        Path::TurnRight
                    } else {
                        Path::TurnLeft
                    })
                }
                _ => num = num * 10 + c.to_digit(10).unwrap() as usize,
            }
        }
        if num != 0 {
            path.push(Path::Move(num));
        }
        (board, path, sp[2].parse::<usize>().unwrap())
    };

    println!("{}", solve1(&board, &path));
    println!("{}", solve2(&board, &path, square_size));
}
