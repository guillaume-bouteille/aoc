

use std::io;
use std::collections::HashSet;

type Inputs = Vec<Vec<u32>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(line.chars().map(|c| c.to_digit(10).expect("bla")).collect::<Vec<u32>>());
        }
    }
    v
}

fn walk(inputs: &Inputs, i: i64, j: i64) -> (i64,i64) {
    let mut h : HashSet<(usize,usize)> = HashSet::new();
    let dirs = vec![ (1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut v = vec![ (i,j) ];
    for k in 0..9 {
        let mut new_v = Vec::new();
        for (ni, nj) in v {
            for (di, dj) in &dirs {
                let new_i = ni+di;
                let new_j = nj+dj;
                if new_i >= 0 && (new_i as usize) < inputs.len() && new_j >= 0 && ((new_j) as usize) < inputs[0].len() {
                    if inputs[(new_i) as usize][(new_j) as usize] == k+1 {
                        new_v.push((new_i, new_j));
                    }
                }
            }
        }
        v = new_v;
    }
    for (t1,t2) in &v {
        h.insert((*t1 as usize, *t2 as usize));
    }
    (h.len() as i64, v.len() as i64)
}

fn solve_part_1(inputs: Inputs) -> i64 {
    let mut res = 0;
    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            if inputs[i][j] == 0 {
                let (t, _) = walk(&inputs, i as i64, j as i64);
                res += t;
            }
        }
    }
    res
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut res = 0;
    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            if inputs[i][j] == 0 {
                let (_, t) = walk(&inputs, i as i64, j as i64);
                res += t;
            }
        }
    }
    res
}


fn main() {

    let inputs = parse_inputs();

    let p1 = solve_part_1(inputs.clone());
    let p2 = solve_part_2(inputs);

    println!("P1={}", p1);
    println!("P2={}", p2);
}
