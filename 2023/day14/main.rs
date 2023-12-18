

use std::io;

type Inputs = Vec<Vec<char>>;

fn parse_inputs() -> Inputs {
    let mut v : Inputs = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(line.chars().collect::<Vec<char>>());
        }
    }
    v
}

fn roll_north(inputs: &mut Inputs) {
    for x in 0..inputs[0].len() {
        for y in 0..inputs.len() {
            if inputs[y][x] == 'O' {
                let mut new_y = y;
                loop {
                    if new_y == 0 {
                        break
                    }
                    if inputs[new_y-1][x] == '#' || inputs[new_y-1][x] == 'O' {
                        break
                    }
                    new_y -= 1
                }
                
                inputs[y][x] = '.';
                inputs[new_y][x] = 'O';
            }
        }
    }
}

fn roll_south(inputs: &mut Inputs) {
    for x in 0..inputs[0].len() {
        for y_m in 0..inputs.len() {
            let y = inputs.len()-1-y_m;
            if inputs[y][x] == 'O' {
                let mut new_y = y;
                loop {
                    if new_y == inputs.len()-1 {
                        break
                    }
                    if inputs[new_y+1][x] == '#' || inputs[new_y+1][x] == 'O' {
                        break
                    }
                    new_y += 1
                }
                
                inputs[y][x] = '.';
                inputs[new_y][x] = 'O';
            }
        }
    }
}

fn roll_east(inputs: &mut Inputs) {
    for y in 0..inputs.len() {
        for x_m in 0..inputs[0].len() {
            let x = inputs[0].len()-1-x_m;
            if inputs[y][x] == 'O' {
                let mut new_x = x;
                loop {
                    if new_x == inputs[0].len()-1 {
                        break
                    }
                    if inputs[y][new_x+1] == '#' || inputs[y][new_x+1] == 'O' {
                        break
                    }
                    new_x += 1
                }
                
                inputs[y][x] = '.';
                inputs[y][new_x] = 'O';
            }
        }
    }
}


fn roll_west(inputs: &mut Inputs) {
    for y in 0..inputs.len() {
        for x in 0..inputs[0].len() {
            if inputs[y][x] == 'O' {
                let mut new_x = x;
                loop {
                    if new_x == 0 {
                        break
                    }
                    if inputs[y][new_x-1] == '#' || inputs[y][new_x-1] == 'O' {
                        break
                    }
                    new_x -= 1
                }
                
                inputs[y][x] = '.';
                inputs[y][new_x] = 'O';
            }
        }
    }
}

fn get_load(inputs : &Inputs) -> usize {
    let mut res = 0;
    for (i, line) in inputs.iter().enumerate() {
        for c in line {
            if *c == 'O' {
                res += inputs.len()-i;                
            }
        }
    }
    res
}

fn main() {

    let inputs = parse_inputs();

    let mut inputs_p1 = inputs.clone();
    roll_north(&mut inputs_p1);
    let p1 = get_load(&inputs_p1);
    println!("P1={}", p1);

    let mut inputs_p2 = inputs.clone();
    let mut previous_cycles : Vec<Inputs> = Vec::new();
    let mut beg_repeat = 0;
    let mut end_repeat = 0;
    for i in 0..1000000000 {
        roll_north(&mut inputs_p2);
        roll_west(&mut inputs_p2);
        roll_south(&mut inputs_p2);
        roll_east(&mut inputs_p2);
        let repeat_opt = previous_cycles.iter().enumerate().find(|(_,e)| **e==inputs_p2);
        if let Some(repeat) = repeat_opt {
            end_repeat = i;
            beg_repeat = repeat.0;
            break;
        }
        previous_cycles.push(inputs_p2.clone());
    }
    assert!(beg_repeat < end_repeat);
    let last_idx = ((1000000000-beg_repeat)%(end_repeat-beg_repeat))+beg_repeat-1;
    let p2 = get_load(&previous_cycles[last_idx]);
    println!("P2={}", p2);
}
