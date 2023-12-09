

use std::io;

type Inputs = Vec<Vec<i64>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(
                line.split(" ").map(|it| it.parse::<i64>().expect("brrrah") )
                    .collect::<Vec<i64>>());
        }
    }
    v
}

fn main() {

    let inputs = parse_inputs();
    let mut p1 : i64 = 0;
    let mut p2 : i64 = 0;
    for l in inputs {
        // Compute derivative until we find a zero sequence
        let mut deriv : Vec<Vec<i64>> = Vec::new();
        deriv.push(l);
        let mut found_deriv_0 = false;
        while found_deriv_0 == false {
            let new_d = deriv.last().expect("blah")
                .windows(2).map(|spl| spl[1] - spl[0]).collect::<Vec<i64>>();
            found_deriv_0 = new_d.iter().all(|it| *it == 0);
            deriv.push(new_d);
        }
        // Extrapolate next and previous value
        let mut next_val : i64 = 0;
        let mut previous_val : i64 = 0;
        for i in (0..deriv.len()-1).rev() {
            next_val += deriv[i].last().expect("bro");
            previous_val = deriv[i][0] - previous_val;
        }
        p1 += next_val;
        p2 += previous_val;
    }
    println!("P1={}", p1);
    println!("P2={}", p2);
}
