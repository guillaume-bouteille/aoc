
use std::collections::HashMap;
use std::collections::HashSet;
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

fn compute_antinodes_nb(inputs : Inputs, range : Vec<i64>) -> i64 {

    let mut h : HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            let c = inputs[i][j];
            if c != '.' {
                if h.contains_key(&c) {
                    h.get_mut(&c).expect("blab").push((i as i64, j as i64));
                } else {
                    h.insert(c, vec![ (i as i64, j as i64)]);
                }
            }
        }
    }

    let mut antinodes : HashSet<(i64, i64)> = HashSet::new();
    for (_, antennas) in h {
        let num_antennas = antennas.len();
        for x in 0..num_antennas {
            for y in 0..num_antennas {
                if x != y {
                    for z in &range {
                        let a_i = antennas[x].0 + z*(antennas[y].0-antennas[x].0);
                        let a_j = antennas[x].1 + z*(antennas[y].1-antennas[x].1);
                        if a_i >= 0 && a_i < (inputs.len() as i64) && a_j >= 0 && a_j < (inputs[0].len() as i64) {
                            antinodes.insert((a_i, a_j));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    antinodes.len() as i64
}

fn solve_part_1(inputs: Inputs) -> i64 {
    compute_antinodes_nb(inputs, vec![ 2 ])
}

fn solve_part_2(inputs: Inputs) -> i64 {
    compute_antinodes_nb(inputs, (1..1000_i64).collect::<Vec<i64>>())
}

fn main() {

    let inputs = parse_inputs();

    let p1 = solve_part_1(inputs.clone());
    let p2 = solve_part_2(inputs);

    println!("P1={}", p1);
    println!("P2={}", p2);
}
