

use std::io;

type Inputs = Vec<(i64, Vec<i64>)>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut it = line.split(": ");
            let result = it.next().expect("prout").parse::<i64>().expect("blahb");
            let v2 = it.next().expect("truc").split_whitespace().map(|e| e.parse::<i64>().expect("balah")).collect::<Vec<i64>>();
            v.push((result, v2))
        }
    }
    v
}

fn is_solvable(expected_total: i64, values: &Vec<i64>) -> bool {
    let nb_combin = 1<<(values.len()-1);
    
    for i in 0..nb_combin {
        let mut total = values[0];
        for (j, v) in values[1..].iter().enumerate() {
            if (i >> j) & 0b1 > 0 {
                total *= v;
            } else {
                total += v;
            }
        }
        if total == expected_total {
            return true;
        }
    }
    false
}

fn concat(total: i64, v: i64) -> i64 {
    let l = ((v as f64).log10() as i64)+1;
    total * 10_i64.pow(l as u32) + v
}

fn is_solvable_p2(expected_total: i64, values: &Vec<i64>) -> bool {
    let nb_combin = 3_i64.pow((values.len()-1) as u32);
    for i in 0..nb_combin {
        let mut total = values[0];
        let mut cursor = i;
        for v in values[1..].iter() {
            match cursor % 3 {
                0 => {total += v;},
                1 => {total *= v;},
                2 => {total = concat(total, *v);}
                _ => todo!("what?")
            };
            cursor /= 3;
        }
        if total == expected_total {
            return true;
        }
    }
    false
}

fn solve_part_1(inputs: Inputs) -> i64 {
    let mut res = 0;
    for (total,values) in inputs {
        if is_solvable(total, &values) {
            res += total;
        }
    }
    res
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut res = 0;
    for (total,values) in inputs {
        if is_solvable(total, &values) {
            res += total;
        } else if is_solvable_p2(total, &values) {
            res += total;
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
