

use std::io;

type Inputs = Vec<Vec<i64>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let v2 = line.split_whitespace().map(|it| it.parse::<i64>().expect("woups") ).collect::<Vec<i64>>();
            v.push(v2);
        }
    }
    v
}

fn is_level_safe(level: &Vec<i64>) -> bool {
    let mut increase_allowed = true;
    let mut decrease_allowed = true;
    for level_slice in level.windows(2) {
        let it1 = level_slice[0];
        let it2 = level_slice[1];
        if it1 == it2 || (it1-it2).abs() > 3 {
            return false;
        }
        if it1 < it2 {
            if !increase_allowed {
                return false;
            }
            decrease_allowed = false;
        }
        if it1 > it2 {
            if !decrease_allowed {
                return false;
            }
            increase_allowed = false;
        }
    }
    return true;
}

fn is_level_safe_p2(level: &Vec<i64>) -> bool {

    if is_level_safe(&level) {
        return true;
    }

    for i in 0..level.len() {
        let new_level = level.iter().enumerate().filter(|(j,_)| *j != i).map(|(_,v)| *v).collect::<Vec<i64>>();
        if is_level_safe(&new_level) {
            return true;
        }
    }
    false
}

fn solve_part_1(inputs: Inputs) -> i64 {
    let mut res : i64 = 0;
    for level in inputs {
        if is_level_safe(&level) {
            res += 1;
        }
    }
    res
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut res : i64 = 0;
    for level in inputs {
        if is_level_safe_p2(&level) {
            res += 1;
        }
    }
    res
}


fn main() {

    let inputs = parse_inputs();

    let p1 = solve_part_1(inputs.clone());
    let p2 = solve_part_2(inputs.clone());

    println!("P1={}", p1);
    println!("P2={}", p2);
}
