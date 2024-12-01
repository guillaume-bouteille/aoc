

use std::io;

type Inputs = (Vec<i64>, Vec<i64>);

fn parse_inputs() -> Inputs {
    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let t = line.split_whitespace().map(|it| it.parse::<i64>().expect("woups") ).collect::<Vec<i64>>();
            assert!(t.len() == 2);
            c1.push(t[0]);
            c2.push(t[1]);
        }
    }
    (c1,c2)
}

fn compute_part1(inputs : Inputs) -> i64 {
    let mut res = 0;

    let (mut v1, mut v2) = inputs;
    v1.sort();
    v2.sort();

    assert!(v1.len() == v2.len());
    for i in 0..v1.len() {
        res += (v1[i] - v2[i]).abs();
    }
    res
}

fn compute_part2(inputs : Inputs) -> i64 {
    let mut res = 0;

    // Not the most elegant solution, but, well, result is obtained in 2ms...
    for i in &inputs.0 {
        let count = inputs.1.iter().filter(|it| {*it == i}).count();
        res += i * (count as i64);
    }
    res
}


fn main() {

    let inputs = parse_inputs();

    let p1 = compute_part1(inputs.clone());

    let p2 = compute_part2(inputs.clone());

    println!("P1={}", p1);
    println!("P2={}", p2);
}
