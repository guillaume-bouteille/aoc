

use std::io;
use std::cmp;
use std::collections::HashMap;

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
                m5.extend(&m[..]);
                m5.push('?');
                d5.extend(&d[..]);
            }
            m5.extend(&m[..]);
            d5.extend(&d[..]);
            v1.push((m, d));
            v2.push((m5, d5));
        }
    }
    (v1,v2)
}

fn visit_possibilities_v2<'a, 'b>(
    map: &'a Vec<char>, defective_groups: &'a [i64], start_idx: usize,
    cache: &'b mut HashMap<(&'a [char], &'a [i64]), i64>) -> i64 {

    if defective_groups.len() == 0 {
        for idx in start_idx..map.len() {
            if map[idx] == '#' {
                return 0;
            }
        }
        return 1;
    }

    if let Some(sol) = cache.get(&(&map[start_idx..],defective_groups)) {
        return *sol;
    }

    let mut s_min : usize = start_idx;
    loop {
        if s_min >= map.len() {
            return 0;
        }
        if map[s_min] != '.' {
            break;
        }
        s_min += 1;
    }

    let next_group_size = defective_groups[0] as usize;
    let new_defective_groups = &defective_groups[1..];

    let space_occupied: i64 = defective_groups.iter().sum::<i64>()
        + (defective_groups.len() as i64) - 1;

    let mut s_max : usize = cmp::min(
        map.len()-1,
        cmp::max(0, (map.len() as i64)-space_occupied) as usize);

    let first_defect = map.iter().enumerate().find(|(i,c)| *i>=start_idx && **c =='#');
    if let Some(fd) = first_defect {
        s_max = cmp::min(s_max, fd.0);
    }

    let mut sum : i64 = 0;
    for s in s_min..s_max+1 {
        let mut is_good = true;
        for i in 0..next_group_size {
            if map[s+i] == '.' {
                is_good = false;
            }
        }
        if !(s+next_group_size == map.len() || map[s+next_group_size] != '#') {
            is_good = false;
        }
        if is_good {
            let new_start_index = s+(next_group_size+1) as usize;
            sum += visit_possibilities_v2(map, &new_defective_groups, new_start_index, cache);
        }
    }

    cache.insert((&map[start_idx..],defective_groups), sum);

    sum
}

fn compute_nb_possibilities<'a>(map: &'a Vec<char>, defective_groups: &'a Vec<i64>) -> i64 {

    let mut cache = HashMap::new();
    visit_possibilities_v2(map, &defective_groups[..], 0, &mut cache)
}

fn main() {

    let (inputs_p1,inputs_p2) = parse_inputs();

    let mut p1 : i64 = 0;
    let mut p2 : i64 = 0;
    for l in inputs_p1.iter() {
        p1 += compute_nb_possibilities(&l.0, &l.1);
    }
    for l in inputs_p2.iter() {
        p2 += compute_nb_possibilities(&l.0, &l.1);
    }
    println!("P1={}", p1);
    println!("P2={}", p2);
}
