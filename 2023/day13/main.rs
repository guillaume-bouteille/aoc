

use std::io;
use std::cmp;

type Pattern = Vec<Vec<char>>;
type Inputs = Vec<Pattern>;


fn parse_inputs() -> Inputs {
    let mut v : Inputs = Vec::new();
    let mut tmp : Pattern = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if line == "" {
                v.push(tmp);
                tmp = Vec::new();
            } else {
                tmp.push( line.chars().collect::<Vec<char>>() );
            }
        }
    }
    v
}

fn find_horizontal_reflection(pattern: &Pattern, expected_nb_smudge : i32) -> usize {
    assert!(pattern.len() > 2);
    for i in 0..pattern.len()-1 {
        let nb_tries = cmp::min(i, pattern.len()-i-2);
        let mut nb_smudge = 0;
        for j in 0..nb_tries+1 {
            for k in 0..pattern[0].len() {
                if pattern[i+j+1][k] != pattern[i-j][k] {
                    nb_smudge += 1;
                }
            }
        }
        if nb_smudge == expected_nb_smudge {
            return i+1;
        }
    }
    0
}

fn find_vertical_reflection(pattern: &Pattern, expected_nb_smudge : i32) -> usize {
    assert!(pattern.len() > 2);
    assert!(pattern[0].len() > 2);
    for i in 0..pattern[0].len()-1 {
        let nb_tries = cmp::min(i, pattern[0].len()-i-2);
        let mut nb_smudge = 0;
        for j in 0..nb_tries+1 {
            for k in 0..pattern.len() {
                if pattern[k][i+j+1] != pattern[k][i-j] {
                    nb_smudge += 1;
                }
            }
        }
        if nb_smudge == expected_nb_smudge {
            return i+1;
        }
    }
    0
}

fn main() {

    let inputs = parse_inputs();

    let mut p1 : usize = 0;
    let mut p2 : usize = 0;
    for pattern in inputs.iter() {
        let rows = find_horizontal_reflection(&pattern, 0);
        let cols = find_vertical_reflection(&pattern, 0);
        assert!(rows != 0 || cols != 0);
        p1 += (rows * 100) + cols;
        let rows_p2 = find_horizontal_reflection(&pattern, 1);
        let cols_p2 = find_vertical_reflection(&pattern, 1);
        assert!(rows_p2 != 0 || cols_p2 != 0);
        p2 += (rows_p2 * 100) + cols_p2;
    }

    println!("P1={}", p1);
    println!("P2={}", p2);
}
