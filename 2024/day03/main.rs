
use std::io;

type Inputs = String;

fn parse_inputs() -> Inputs {
    let mut res = String::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            res += &(line.to_string() + "\n").to_string();
        }
    }
    return res;
}

// Look for a pattern mul(x,y) in a string, where x and y are digit
// return value is tuple where
// first element is the index shift to apply on search string for the next search
// second element is the extracted multiplied value (0 if no extract)
fn look_for_mul(search_str : &str) -> (usize, i64) {
    if search_str.len() < 4 {
        return (4, 0);
    }

    if search_str[0..4] != *"mul(" {
        return (1, 0);
    }

    let mut idx = 4;
    while idx < search_str.len() {
        let c = search_str.bytes().nth(idx).expect("prout");
        if c == b',' {
            break;
        } else if !c.is_ascii_digit() {
            return (idx, 0);
        } else {
            idx += 1;
        }
    }

    let first_num = search_str[4..idx].parse::<i64>().expect("paf!");
    idx+=1;

    let mut idx2 = idx;
    while idx2 < search_str.len() {
        let c = search_str.bytes().nth(idx2).expect("prout");
        if c == b')' {
            break;
        } else if !c.is_ascii_digit() {
            return (idx2, 0);
        } else {
            idx2 += 1;
        }
    }
    let second_num = search_str[idx..idx2].parse::<i64>().expect("paf!");
    
    (idx2, second_num * first_num)
}

fn solve_part_1(inputs : Inputs) -> i64 {

    let mut idx = 0;
    let mut res = 0;
    while idx < inputs.len() {
        let (shift, res_it) = look_for_mul(&inputs[idx..]);
        idx += shift;
        res += res_it;
    }
    res
}

fn solve_part_2(inputs : Inputs) -> i64 {
    let mut idx = 0;
    let mut res = 0;
    let mut enabled = true;
    while idx < inputs.len() {
        if idx < inputs.len() - 7 && inputs[idx..idx+7] == *"don't()" {
            idx += 7;
            enabled = false;
        } else if idx < inputs.len() - 4 && inputs[idx..idx+4] == *"do()" {
            idx += 4;
            enabled = true;
        } else {
            let (shift, res_it) = look_for_mul(&inputs[idx..]);
            idx += shift;
            if enabled {
                res += res_it;
            }
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
