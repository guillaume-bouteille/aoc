

use std::io;

type PipeMap = Vec<Vec<char>>;
type Position = (usize, usize);


#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum OffsetPosition {
    Middle = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
    UpLeft = 5,
    UpRight = 6,
    DownLeft = 7,
    DownRight = 8,
}

type Inputs = (PipeMap, Position, (Position,Position));

fn parse_inputs() -> Inputs {
    let mut pipe_map = Vec::new();
    let mut start_pos : (usize,usize) = (0,0);
    for (j, line_it) in io::stdin().lines().enumerate() {
        if let Ok(line) = line_it {
            let chars = line.chars().collect::<Vec<char>>();
            for (i,c) in chars.iter().enumerate() {
                if *c == 'S' {
                    start_pos = (j, i);
                }
            }
            pipe_map.push(chars);
        }
    }
    // Don't want to manage edge cases
    assert!(start_pos.0 > 0);
    assert!(start_pos.0 < pipe_map.len()-1);
    assert!(start_pos.1 > 0);
    assert!(start_pos.1 < pipe_map[0].len()-1);

    let left_pos = (start_pos.0, start_pos.1-1);
    let left = pipe_map[start_pos.0][start_pos.1-1];
    let right_pos = (start_pos.0, start_pos.1+1);
    let right = pipe_map[start_pos.0][start_pos.1+1];
    let up_pos = (start_pos.0-1, start_pos.1);
    let up = pipe_map[start_pos.0-1][start_pos.1];
    let down_pos = (start_pos.0+1, start_pos.1);
    let down = pipe_map[start_pos.0+1][start_pos.1];

    let mut start_pipes = Vec::new();
    if left == 'L' || left == '-' || left == 'F' {
        start_pipes.push(left_pos);
    }
    if right == '7' || right == '-' || right == 'J' {
        start_pipes.push(right_pos);
    }
    if up == '7' || up == '|' || up == 'F' {
        start_pipes.push(up_pos);
    }
    if down == 'L' || down == '|' || down == 'J' {
        start_pipes.push(down_pos);
    }

    assert!(start_pipes.len() == 2);
    (pipe_map, start_pos, (start_pipes[0], start_pipes[1]))
}

fn compute_next_pipe(pipe_map: &PipeMap, current_position: Position, next_position: Position) -> Position {

    let delta_in : (i64, i64) = ((next_position.0 as i64)-(current_position.0 as i64), (next_position.1 as i64)-(current_position.1 as i64));
    let delta_out : (i64, i64) = match (delta_in, pipe_map[next_position.0][next_position.1]) {
        // | is a vertical pipe connecting north and south.
        ((1,0), '|') => (1,0),
        ((-1,0), '|') => (-1,0),
        // - is a horizontal pipe connecting east and west.
        ((0,1), '-') => (0,1),
        ((0,-1), '-') => (0,-1),
        // L is a 90-degree bend connecting north and east.
        ((1,0), 'L') => (0,1),
        ((0,-1), 'L') => (-1,0),
        // J is a 90-degree bend connecting north and west.
        ((1,0), 'J') => (0,-1),
        ((0,1), 'J') => (-1,0),
        // 7 is a 90-degree bend connecting south and west.
        ((-1,0), '7') => (0,-1),
        ((0,1), '7') => (1,0),
        // F is a 90-degree bend connecting south and east.
        ((-1,0), 'F') => (0,1),
        ((0,-1), 'F') => (1,0),
        _ => todo!()
    };
    ((((next_position.0 as i64)+delta_out.0) as usize), (((next_position.1 as i64)+delta_out.1) as usize))
}


type ElephantPos = ((i64, i64), OffsetPosition);

fn can_escape(pipe_map: &PipeMap, start_position: ElephantPos) -> (bool,Vec<ElephantPos>) {

    let mut visited_pos : Vec<ElephantPos> = Vec::new();
    let mut next_positions : Vec<ElephantPos> = Vec::new();
    next_positions.push(start_position);

    while next_positions.len() > 0 {
        let np = next_positions.remove(0);
        if visited_pos.contains(&np) == true {
            continue;
        }

        if np.0.0 < 0 || np.0.0 >= pipe_map.len() as i64 || np.0.1 < 0 || np.0.1 >= pipe_map[0].len() as i64 {
            return (true,visited_pos); // We escaped!!!
        }

        let mut candidates : Vec<ElephantPos> = Vec::new();

        let pipe = pipe_map[np.0.0 as usize][np.0.1 as usize];

        match np {
            (p, OffsetPosition::Middle) => {
                candidates.extend([(p, OffsetPosition::Left), (p, OffsetPosition::Right), (p, OffsetPosition::Up), (p, OffsetPosition::Down), (p, OffsetPosition::UpLeft), (p, OffsetPosition::UpRight), (p, OffsetPosition::DownLeft), (p, OffsetPosition::DownRight)])
            },
            (p, OffsetPosition::Left) => {
                candidates.push(((p.0, p.1-1), OffsetPosition::Right));
                match pipe {
                    '|' | 'F' | 'L' => candidates.extend([(p, OffsetPosition::UpLeft), (p, OffsetPosition::DownLeft)]),
                    'S' => {},
                    // '.' normal => back to middle
                    '.' => candidates.push((p, OffsetPosition::Middle)),
                    // '-','J','7' strange => back to middle
                    _ => { candidates.push((p, OffsetPosition::Middle)) },
                }
            },
            (p, OffsetPosition::Right) => {
                candidates.push(((p.0, p.1+1), OffsetPosition::Left));
                match pipe {
                    '|' | 'J' | '7' => candidates.extend([(p, OffsetPosition::UpRight), (p, OffsetPosition::DownRight)]),
                    'S' => {},
                    // '.' normal => back to middle
                    '.' => candidates.push((p, OffsetPosition::Middle)),
                    // '-','F','L' strange => back to middle
                    _ => { candidates.push((p, OffsetPosition::Middle)) },
                }
            },
            (p, OffsetPosition::Up) => {
                candidates.push(((p.0-1, p.1), OffsetPosition::Down));
                match pipe {
                    '-' | 'F' | '7' => candidates.extend([(p, OffsetPosition::UpRight), (p, OffsetPosition::UpLeft)]),
                    'S' => {},
                    // '.' normal => back to middle
                    '.' => candidates.push((p, OffsetPosition::Middle)),
                    // '-','J','L' strange => back to middle
                    _ => { candidates.push((p, OffsetPosition::Middle)) },
                }
            },
            (p, OffsetPosition::Down) => {
                candidates.push(((p.0+1, p.1), OffsetPosition::Up));
                match pipe {
                    '-' | 'J' | 'L' => candidates.extend([(p, OffsetPosition::DownRight), (p, OffsetPosition::DownLeft)]),
                    'S' => {},
                    // '.' normal => back to middle
                    '.' => candidates.push((p, OffsetPosition::Middle)),
                    // '-','F','7' strange => back to middle
                    _ => { candidates.push((p, OffsetPosition::Middle)) },
                }
            },
            (p, OffsetPosition::UpLeft) => {
                 candidates.extend([((p.0-1, p.1), OffsetPosition::DownLeft), ((p.0, p.1-1), OffsetPosition::UpRight)]);
                match pipe {
                    'J' => {},
                    '-'|'7' => candidates.push((p, OffsetPosition::Up)),
                    '|'|'L' => candidates.push((p, OffsetPosition::Left)),
                    'F' => candidates.extend([(p, OffsetPosition::Up), (p, OffsetPosition::Left)]),
                    'S' => {},
                    // '.' normal => back to middle
                    _ => candidates.push((p, OffsetPosition::Middle)),
                }
            },
            (p, OffsetPosition::UpRight) => {
                candidates.extend([((p.0-1, p.1), OffsetPosition::DownRight), ((p.0, p.1+1), OffsetPosition::UpLeft)]);
                match pipe {
                    'L' => {},
                    '-'|'F' => candidates.push((p, OffsetPosition::Up)),
                    '|'|'J' => candidates.push((p, OffsetPosition::Right)),
                    '7' => candidates.extend([(p, OffsetPosition::Up), (p, OffsetPosition::Right)]),
                    'S' => {},
                    // '.' normal => back to middle
                    _ => candidates.push((p, OffsetPosition::Middle)),
                }
            },
            (p, OffsetPosition::DownLeft) => {
                candidates.extend([((p.0+1, p.1), OffsetPosition::UpLeft), ((p.0, p.1-1), OffsetPosition::DownRight)]);
                match pipe {
                    '7' => {},
                    '|'|'F' => candidates.push((p, OffsetPosition::Left)),
                    '-'|'J' => candidates.push((p, OffsetPosition::Down)),
                    'L' => candidates.extend([(p, OffsetPosition::Left), (p, OffsetPosition::Down)]),
                    'S' => {},
                    // '.' normal => back to middle
                    _ => candidates.push((p, OffsetPosition::Middle)),
                }
            },
            (p, OffsetPosition::DownRight) => {
                candidates.extend([((p.0+1, p.1), OffsetPosition::UpRight), ((p.0, p.1+1), OffsetPosition::DownLeft)]);
                match pipe {
                    'F' => {},
                    '|'|'7' => candidates.push((p, OffsetPosition::Right)),
                    '-'|'L' => candidates.push((p, OffsetPosition::Down)),
                    'J' => candidates.extend([(p, OffsetPosition::Right), (p, OffsetPosition::Down)]),
                    'S' => {},
                    // '.' normal => back to middle
                    _ => candidates.push((p, OffsetPosition::Middle)),
                }
            },
        }


        for c in candidates {
            if visited_pos.contains(&c) == false {
                next_positions.push(c);
            }
        }
        visited_pos.push(np);
    }
    (false,visited_pos) // No escape!!!
}

fn main() {

    let (pipe_map, start_pos, start_pipes) = parse_inputs();


    let mut current_position = start_pos;
    let mut next_position = start_pipes.0;
    let mut count_p1 : i64 = 0;
    while next_position != start_pos {
        let t = compute_next_pipe(&pipe_map, current_position, next_position);
        current_position = next_position;
        next_position = t;
        count_p1 += 1;
    }
    println!("P1={}", (count_p1+1)/2);

    let mut ground_tiles = Vec::new();
    for (j,v) in pipe_map.iter().enumerate() {
        for (i,c) in v.iter().enumerate() {
            if *c == '.' {
                ground_tiles.push((j,i));
            }
        }
    }
    let mut vis : Vec<ElephantPos> = Vec::new();
    for init in vec![OffsetPosition::Left, OffsetPosition::Right, OffsetPosition::Down, OffsetPosition::Up] {
        let (can_escape_b, vis_b) = can_escape(
            &pipe_map, (((start_pos.0 as i64), (start_pos.1 as i64)), init));
        if can_escape_b == false {
            vis = vis_b;
            break;
        }
    }
    let visited_positions = vis.iter().filter(|it| it.1 == OffsetPosition::Middle).map(|v| ((v.0.0 as usize), (v.0.1 as usize))).collect::<Vec<Position>>();
    let count_p2 = visited_positions.len();
    println!("P2={}", count_p2);
}
