

use std::io;

type Line = (Vec<char>, Vec<i64>);
type Inputs = Vec<Line>;

fn parse_inputs() -> (Inputs,Inputs) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let (map_l, nb_l) = line.split_once(" ").expect("youhou");
            let m = map_l.chars().collect::<Vec<char>>();
            let d = nb_l.split(",").map(|s| s.parse::<i64>().expect("tuttu") ).collect::<Vec<i64>>();
            let mut m5 = Vec::new();
            let mut d5 = Vec::new();
            for _ in 0..4 {
                m5.extend(m.clone());
                m5.push('?');
                d5.extend(d.clone());
            }
            m5.extend(m.clone());
            d5.extend(d.clone());
            v1.push((m, d));
            v2.push((m5, d5));
            
        }
    }
    (v1,v2)
}

fn is_valid_solution(map: &Vec<char>, defective_groups: &Vec<i64>, end_lookup : usize) -> bool {

    let mut nb_curr : i64 = 0;
    let mut defective_groups_idx = 0;
    assert!(defective_groups.len()>0);
    for (i,c) in map.iter().enumerate() {
        if i >= end_lookup {
            return true;
        }
        if *c == '#' {
            nb_curr += 1;
        } else {
            if nb_curr > 0 {
                if defective_groups_idx >= defective_groups.len() {
                    return false;
                }
                if defective_groups[defective_groups_idx] != nb_curr {
                    return false;
                }
                defective_groups_idx += 1;
            }
            nb_curr = 0;
        }
    }
    if nb_curr > 0 {
        if defective_groups_idx >= defective_groups.len() {
            return false;
        }
        if defective_groups[defective_groups_idx] != nb_curr {
            return false;
        }
    }
    true
}

fn visit_possibilities(map: &Vec<char>, defective_groups: &Vec<i64>, nb_choices : i64, unknown_positions: &Vec<usize>) -> i64 {

    let mut sum : i64 = 0;

    if unknown_positions.len() > 0 {

        if nb_choices > 0 {
            let mut nmap = map.clone();
            let mut nup = unknown_positions.clone();
            let idx = nup.remove(0);
            nmap[idx] = '#';
            // only go down if still valid at this point
            if is_valid_solution(&nmap, defective_groups, idx) {
                sum += visit_possibilities(&nmap, defective_groups, nb_choices-1, &nup);
            }
        }
        let mut nmap = map.clone();
        let mut nup = unknown_positions.clone();
        let idx = nup.remove(0);
        nmap[idx] = '.';
        // only go down if still valid at this point
        if is_valid_solution(&nmap, defective_groups, idx) {
            sum += visit_possibilities(&nmap, defective_groups, nb_choices, &nup);
        }
    } else {
        if nb_choices == 0 {
            let iv = is_valid_solution(map, defective_groups, map.len());
            //eprintln!("--- {:?} is {}", map, iv);
            if iv {
                sum = 1;
            }
        }
    }

    sum
}

fn compute_nb_possibilities(map: &Vec<char>, defective_groups: &Vec<i64>) -> i64 {

    let nb_defective : i64 = defective_groups.iter().sum();
    let nb_known_defectives : i64 = map.iter().filter(|c| **c == '#').count() as i64;

    let unknown_positions = map.iter().enumerate().filter(|(_,c)| **c == '?')
        .map(|(i,_)| i).collect::<Vec<usize>>();

    visit_possibilities(map, defective_groups, nb_defective-nb_known_defectives, &unknown_positions)
}

fn main() {

    let (inputs_p1,inputs_p2) = parse_inputs();

    let mut p1 : i64 = 0;
    let mut p2 : i64 = 0;
    for l in inputs_p1 {
        p1 += compute_nb_possibilities(&l.0, &l.1);
    }

    for l in inputs_p2 {
        //eprintln!("{:?} {:?}", l.0, l.1);
        p2 += compute_nb_possibilities(&l.0, &l.1);
    }
    println!("P1={}", p1);
    println!("P2={}", p2);
}
