
use std::io;

type Inputs = (usize, Vec<u8>);

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    let mut line_len = 0;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if line_len == 0 {
                line_len = line.len();
            }
            v.extend(line.bytes().collect::<Vec<u8>>());
        }
    }
    (line_len, v)
}

fn get_start_pos(inputs: &Inputs) -> (usize,usize)
{
    for (i,c) in inputs.1.iter().enumerate() {
        if *c == b'^' {
            return (i/inputs.0,i%inputs.0);
        }
    }
    assert!(false);
    (0,0)
}

fn get_nb_visits(inputs: &mut Inputs, start_x: usize, start_y: usize ) -> i64 {
    let mut x = start_x;
    let mut y = start_y;

    let line_len = inputs.0;
    let col_len = inputs.1.len() / line_len;
    let mut visited : Vec<u8> = vec![ 0; inputs.1.len()];
    let mut c = inputs.1[x*line_len+y];
    loop {
        let mask = match c {
            b'^' => 0b0001,
            b'>' => 0b0010,
            b'v' => 0b0100,
            b'<' => 0b1000,
            _ => {assert!(false); 0}
        };
        if visited[x*line_len+y] & mask > 0 {
            return 0;
        } else {
            visited[x*line_len+y] |= mask;
        }
        inputs.1[x*line_len+y] = b'x';
        match c {
            b'^' => {
                if x == 0 {
                    break;
                } else {
                    if inputs.1[(x-1)*line_len+y] == b'#' {
                        c = b'>';
                    } else {
                        x = x-1;
                    }
                }
            },
            b'>' => {
                if y == (line_len-1) {
                    break;
                } else {
                    if inputs.1[x*line_len+y+1] == b'#' {
                        c = b'v';
                    } else {
                        y = y + 1;
                    }
                }
            },
            b'v' => {
                if x == (col_len-1) {
                    break;
                } else {
                    if inputs.1[(x+1)*line_len+y] == b'#' {
                        c = b'<';
                    } else {
                        x = x + 1;
                    }
                }
            },
            b'<' => {
                if y == 0 {
                    break;
                } else {
                    if inputs.1[x*line_len+y-1] == b'#' {
                        c = b'^';
                    } else {
                        y = y - 1;
                    }
                }
            }
            _ => { eprintln!("c={}", c); assert!(false); }
        }
    }

    let mut res = 0;
    for c in visited {
        if c > 0 {
            res += 1;
        }
    }
    res
}

fn solve_part_1(inputs: &mut Inputs) -> i64 {
    let (x, y) = get_start_pos(inputs);
    get_nb_visits(inputs, x, y)
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut p1_input = inputs.clone();
    let (x_start, y_start) = get_start_pos(&inputs);
    get_nb_visits(&mut p1_input, x_start, y_start);
    let mut res = 0;
    for i in 0..inputs.1.len() {
        let x = i / inputs.0;
        let y = i % inputs.0;
        if (x_start != x || y_start != y) && p1_input.1[x*inputs.0+y] == b'x' {
            let mut p2_input = inputs.clone();
            p2_input.1[x*inputs.0+y] = b'#';
            if get_nb_visits(&mut p2_input, x_start, y_start) == 0 {
                res += 1;
            }
        }
    }
    res
}

fn main() {

    let inputs = parse_inputs();

    let p1 = solve_part_1(&mut inputs.clone());
    let p2 = solve_part_2(inputs);

    println!("P1={}", p1);
    println!("P2={}", p2);
}
