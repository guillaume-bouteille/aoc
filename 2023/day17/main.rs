

use std::io;
use std::collections::HashSet;

type Inputs = Vec<Vec<i64>>;

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push( line.chars().map(|i| (i as i64) - ('0' as i64) ).collect() );
        }
    }
    v
}

type Position = (usize,usize,char,usize);

type PathCandidatesCallback = fn(inputs: &Inputs, p: &Position, score : i64) -> Vec<(i64,Position)>;

fn get_path_candidates(inputs: &Inputs, p: &Position, score : i64) -> Vec<(i64,Position)> {
    let mut candidates = Vec::new();
    if p.0 > 0 && p.2 != '>' && (p.2 != '<' || p.3 < 3) {
        candidates.push( (
                score + inputs[p.1][p.0-1],
                (p.0-1, p.1, '<', if p.2 == '<' {p.3+1} else {1})
        ));
    }
    if p.0 < inputs[0].len()-1 && p.2 != '<' && (p.2 != '>' || p.3 < 3) {
        candidates.push( (
                score + inputs[p.1][p.0+1],
                (p.0+1, p.1, '>', if p.2 == '>' {p.3+1} else {1})
        ));
    }
    if p.1 > 0 && p.2 != 'v' && (p.2 != '^' || p.3 < 3) {
        candidates.push( (
                score + inputs[p.1-1][p.0],
                (p.0, p.1-1, '^', if p.2 == '^' {p.3+1} else {1})
        ));
    }
    if p.1 < inputs.len()-1 && p.2 != '^' && (p.2 != 'v' || p.3 < 3) {
        candidates.push( (
                score + inputs[p.1+1][p.0],
                (p.0, p.1+1, 'v', if p.2 == 'v' {p.3+1} else {1})
        ));
    }
    candidates
}

fn get_ultrapath_candidates(inputs: &Inputs, p: &Position, score : i64) -> Vec<(i64,Position)> {
    let mut candidates = Vec::new();
    if p.0 > 0 && p.2 != '>' && (p.3 == 0 || (p.2 != '<' && p.3 >= 4) || (p.2 == '<' && p.3 < 10)) {
        candidates.push( (
                score + inputs[p.1][p.0-1],
                (p.0-1, p.1, '<', if p.2 == '<' {p.3+1} else {1})
        ));
    }
    if p.0 < inputs[0].len()-1 && p.2 != '<' && (p.3 == 0 || (p.2 != '>' && p.3 >= 4) || (p.2 == '>' && p.3 < 10)) {
        candidates.push( (
                score + inputs[p.1][p.0+1],
                (p.0+1, p.1, '>', if p.2 == '>' {p.3+1} else {1})
        ));
    }
    if p.1 > 0 && p.2 != 'v' && (p.3 == 0 || (p.2 != '^' && p.3 >= 4) || (p.2 == '^' && p.3 < 10)) {
        candidates.push( (
                score + inputs[p.1-1][p.0],
                (p.0, p.1-1, '^', if p.2 == '^' {p.3+1} else {1})
        ));
    }
    if p.1 < inputs.len()-1 && p.2 != '^' && (p.3 == 0 || (p.2 != 'v' && p.3 >= 4) || (p.2 == 'v' && p.3 < 10)) {
        candidates.push( (
                score + inputs[p.1+1][p.0],
                (p.0, p.1+1, 'v', if p.2 == 'v' {p.3+1} else {1})
        ));
    }
    candidates
}

fn find_shortest_path(inputs: &Inputs, get_candidates : PathCandidatesCallback) -> i64 {
    let dst = (inputs[0].len()-1, inputs.len()-1);
    let mut visited : HashSet<Position> = HashSet::new();
    let mut next_visits : Vec<(i64,Position)> = Vec::new();
    next_visits.push((0, (0,0,'~',0)));
    while next_visits.len() > 0 {
        eprintln!("{} {}", visited.len(), next_visits.len());
        let (score, p) = next_visits.remove(0);
        if visited.contains(&p) {
            continue;
        }
        if (p.0,p.1) == dst {
            return score;
        }
        visited.insert(p);
        let candidates = get_candidates(inputs, &p, score);

        for c in candidates.iter().filter(|(s,p)| visited.contains(p) == false) {
            /*
            let ev = next_visits.find(|(_,e)| e == c);
            if let Some((score,e)) = ev {
                if c.0 < score {
                    next_visits
                }
            }
            */

            let it = next_visits.iter().enumerate().filter(|(_,(e_score, _))| *e_score >= c.0).next();
            if let Some((i, _)) = it {
                next_visits.insert(i, *c);
            } else {
                next_visits.push(*c);
            }
        }
    }
    0
}

fn main() {

    let inputs = parse_inputs();

    let p1 : i64 = find_shortest_path(&inputs, get_path_candidates);
    println!("P1={}", p1);

    let p2 : i64 = find_shortest_path(&inputs, get_ultrapath_candidates);
    println!("P2={}", p2);
}
