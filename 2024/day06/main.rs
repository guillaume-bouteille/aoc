
use std::collections::HashSet;
use std::io;

type Inputs = Vec<Vec<u8>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(line.bytes().collect::<Vec<u8>>());
        }
    }
    v
}

fn get_start_pos(inputs: &Inputs) -> (usize,usize)
{
    for (i,line) in inputs.iter().enumerate() {
        for (j,c) in line.iter().enumerate() {
            if *c == b'^' {
                return (i,j);
            }
        }
    }
    assert!(false);
    (0,0)
}

fn solve_part_1(inputs: &mut Inputs) -> i64 {

    let (mut x, mut y) = get_start_pos(inputs);

    let mut c = inputs[x][y];
    let mut visited : HashSet<(usize, usize, u8)> = HashSet::new();
    loop {
        //eprintln!("{} {} {}", c, x, y);
        if visited.contains(&(x,y,c)) {
            return 0;
        }
        visited.insert((x,y,c));
        inputs[x][y] = b'x';
        match c {
            b'^' => {
                if x == 0 {
                    break;
                } else {
                    if inputs[x-1][y] == b'#' {
                        c = b'>';
                    } else {
                        x = x-1;
                    }
                }
            },
            b'>' => {
                if y == (inputs[0].len()-1) {
                    break;
                } else {
                    if inputs[x][y+1] == b'#' {
                        c = b'v';
                    } else {
                        y = y + 1;
                    }
                }
            },
            b'v' => {
                if x == (inputs.len()-1) {
                    break;
                } else {
                    if inputs[x+1][y] == b'#' {
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
                    if inputs[x][y-1] == b'#' {
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
    for line in inputs {
        for c in line {
            if *c == b'x' {
                res += 1;
            }
        }
    }
    res
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut p1_input = inputs.clone();
    let (x_start, y_start) = get_start_pos(&inputs);
    solve_part_1(&mut p1_input);
    let mut res = 0;
    for x in 0..inputs.len() {
        for y in 0..inputs[0].len() {
            if (x_start != x || y_start != y) && p1_input[x][y] == b'x' {
                let mut p2_input = inputs.clone();
                p2_input[x][y] = b'#';
                if solve_part_1(&mut p2_input) == 0 {
                    res += 1;
                }
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
