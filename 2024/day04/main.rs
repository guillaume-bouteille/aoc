
use std::io;

type Inputs = Vec<Vec<char>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(line.chars().collect::<Vec<char>>());
        }
    }
    v
}

fn solve_part_1(inputs : Inputs) -> i64 {
    let directions : Vec<(i64,i64)> = vec![(0,1), (0,-1), (-1,0), (1,0), (1,1), (-1,-1), (1,-1), (-1,1)];
    let mut res = 0;
    for (i, line) in inputs.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != 'X' {
                continue
            }

            for (x,y) in &directions {
                if (i as i64)+x*3 < (inputs.len() as i64) && (i as i64)+x*3 >= 0 && ((j as i64)+y*3) < (line.len() as i64) && (j as i64)+y*3 >= 0 {
                    if (1..4).map(|z| inputs[((i as i64)+x*z) as usize][((j as i64)+y*z) as usize]).collect::<String>() == "MAS" {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn solve_part_2(inputs : Inputs) -> i64 {
    let p1 = vec!['M', 'A', 'S'];
    let p2 = vec!['S', 'A', 'M'];
    let mut res = 0;
    for (i, line) in inputs.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if i > 0 && i < inputs.len()-1 && j > 0 && j < line.len()-1 {
                let w1 = vec![inputs[i-1][j-1], *c, inputs[i+1][j+1]];
                let w2 = vec![inputs[i+1][j-1], *c, inputs[i-1][j+1]];

                if (w1 == p1 || w1 == p2) && (w2 == p1 || w2 == p2) {
                    res += 1;
                }
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
