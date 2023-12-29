

use std::io;
use std::cmp;

type InputsP = Vec<(char, i64)>;

type Inputs = (InputsP, InputsP);

fn parse_inputs() -> Inputs {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut spl = line.split(" ");
            v1.push( (
                spl.next().expect("youhou").chars().nth(0).expect("blah"),
                spl.next().expect("wazaah").parse::<i64>().expect("wouh")
            ));
            let color_truc = spl.next().expect("plop");
            let nb_2 = u32::from_str_radix(&color_truc[2..color_truc.len()-2], 16).unwrap() as i64;
            let dir_idx : usize = color_truc.len() - 2;
            let dir_2 = match color_truc.chars().nth(dir_idx).expect("grrr") {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => todo!("???"),
            };
            v2.push((dir_2, nb_2));
        }
    }
    (v1,v2)
}

type Position = (i64,i64);

fn does_intersect(verticals : &Vec<(Position, Position)>, p1 : Position, p2 : Position) -> bool {

    let f1 = verticals.iter().filter(|e| e.0 == p1 || e.1 == p1).next().expect("blah");
    let f2 = verticals.iter().filter(|e| e.0 == p2 || e.1 == p2).next().expect("blah");

    if (f1.0 == p1 && f2.0 == p2) || (f1.1 == p1 && f2.1 == p2) {
        return false;
    } else {
        return true;
    }
}

fn solve(inputs: &InputsP) -> i64 {
    let mut curr_pos : Position = (0,0);
    let mut min_pos : Position = (0,0);
    let mut max_pos : Position = (0,0);
    let mut horizontals : Vec<(Position, Position)> = Vec::new();
    let mut verticals : Vec<(Position, Position)> = Vec::new();
    for (dir, nb_blocks) in inputs.iter() {
        match dir {
            'R' => {
                horizontals.push((curr_pos, (curr_pos.0+nb_blocks, curr_pos.1)));
                curr_pos.0 += nb_blocks;
            },
            'L' => {
                horizontals.push(((curr_pos.0-nb_blocks, curr_pos.1), curr_pos));
                curr_pos.0 -= nb_blocks;
            },
            'U' => {
                verticals.push(((curr_pos.0, curr_pos.1-nb_blocks), curr_pos));
                curr_pos.1 -= nb_blocks;
            },
            'D' => {
                verticals.push((curr_pos, (curr_pos.0, curr_pos.1+nb_blocks)));
                curr_pos.1 += nb_blocks;
            },
            _ => todo!("???"),
        }
        min_pos.0 = cmp::min(min_pos.0, curr_pos.0);
        min_pos.1 = cmp::min(min_pos.1, curr_pos.1);
        max_pos.0 = cmp::max(max_pos.0, curr_pos.0);
        max_pos.1 = cmp::max(max_pos.1, curr_pos.1);
    }
    horizontals.sort_by(|e1,e2| e1.0.1.cmp(&e2.0.1) );
    verticals.sort_by(|e1,e2| e1.0.0.cmp(&e2.0.0));
    let mut sum : i64 = 0;
    let mut y = min_pos.1;
    while y <= max_pos.1 {
        let on_line = horizontals.iter().filter(|e| e.0.1 == y).collect::<Vec<_>>();
        if on_line.len() > 0 {
            let mut x = min_pos.0-1;
            let mut nb_in : i64 = 0;
            let mut is_in = false;
            while x < max_pos.0 {
                let it2 = verticals.iter().filter(|e| y >= e.0.1 && y <= e.1.1 && x < e.0.0).next();
                let x2;
                let mut nb_in_tmp = 0;
                if let Some(itt) = it2 {
                    x2 = itt.0.0;
                } else {
                    x2 = max_pos.0
                }

                if on_line.iter().filter(|e| x2 == e.1.0 ).count() > 0 {
                    if does_intersect(&verticals, (x-1,y), (x2,y)) {
                        nb_in_tmp = (x2-x)+1;
                    } else {
                        nb_in_tmp = (x2-x)+1;
                        is_in = !is_in;
                    }
                } else {
                    if is_in {
                        nb_in_tmp = (x2-x)+1;
                    } else {
                        if it2.is_some() {
                            nb_in_tmp = 1;
                        }
                    }
                    is_in = !is_in;
                }

//                eprintln!("({} {}) {}", x,y,nb_in_tmp);
                nb_in += nb_in_tmp;
                x = x2+1;
            }
//            eprintln!("on_line y={} nb_in={}", y, nb_in);
            sum += nb_in;
            y += 1;
        } else {

            let it = horizontals.iter().filter(|e| e.0.1 > y).next();
            let y2;
            if let Some(itt) = it {
                y2 = itt.0.1;
            } else {
                y2 = max_pos.1;
            }
            let mut nb_in : i64 = 0;
            let mut is_in = false;
            let mut x = min_pos.0-1;

            while x < max_pos.0 {
                let it2 = verticals.iter().filter(|e| y > e.0.1 && y < e.1.1 && x < e.0.0).next();
                let x2;
                if let Some(itt) = it2 {
                    x2 = itt.0.0;
                } else {
                    x2 = max_pos.0
                }

                if is_in {
                    nb_in += (x2-x)+2;
                }
                is_in = !is_in;

                x = x2+1;
            }
//            eprintln!("on lines y=[{} {}[ nb_in={}", y, y2, nb_in);
            sum += nb_in * (y2-y);
            y = y2;
        }
    }
    sum
}

fn main() {

    let (inputs_p1, inputs_p2) = parse_inputs();

    let p1 : i64 = solve(&inputs_p1);
    println!("P1={}", p1);

    let p2 : i64 = solve(&inputs_p2);
    println!("P2={}", p2);
}
