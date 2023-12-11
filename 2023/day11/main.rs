

use std::io;
use std::cmp;

type Position = (i64,i64);

type Inputs = (Vec<Position>,Vec<i64>,Vec<i64>);

fn parse_inputs() -> Inputs {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push(line.chars().collect::<Vec<char>>());
        }
    }

    // Find rows with no galaxy
    let mut empty_rows = Vec::new();
    for y in 0..v.len() {
        if v[y].iter().all(|c| *c == '.') {
            empty_rows.push(y as i64);
        }
    }

    // Find columns with no galaxy
    let mut empty_cols = Vec::new();
    for x in 0..v[0].len() {
        let mut all_empty : bool = true;
        for y2 in 0..v.len() {
            if v[y2][x] != '.' {
                all_empty = false;
            }
        }
        if all_empty {
            empty_cols.push(x as i64);
        }
    }

    // Retrieve all the galaxy positions
    let mut galaxies : Vec<Position> = Vec::new();
    for (y, l) in v.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '#' {
                galaxies.push((x as i64,y as i64));
            }
        }
    }
    (galaxies, empty_rows, empty_cols)
}


fn main() {

    let (galaxies,empty_rows,empty_cols) = parse_inputs();

    let mut p1 : i64 = 0;
    let mut p2 : i64 = 0;
    for (i1, g1) in galaxies.iter().enumerate() {
        for (i2, g2) in galaxies.iter().enumerate() {
            if i1 < i2 {
                let init_dist = (g1.0-g2.0).abs() + (g1.1-g2.1).abs();
                let nb_empty_rows = empty_rows.iter()
                    .filter(|y| **y > cmp::min(g1.1, g2.1) && **y < cmp::max(g1.1, g2.1))
                    .count() as i64; 
                let nb_empty_cols = empty_cols.iter()
                    .filter(|y| **y > cmp::min(g1.0, g2.0) && **y < cmp::max(g1.0, g2.0))
                    .count() as i64;
                p1 += init_dist + nb_empty_rows + nb_empty_cols;
                // -1 because we must not count the original empty row or column
                p2 += init_dist + (nb_empty_rows + nb_empty_cols)*(1000000-1);
            }
        }
    }
    println!("P1={}", p1);
    println!("P2={}", p2);
}
