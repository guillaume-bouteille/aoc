
use std::io;

struct Line {
    p1_start : i32,
    p1_end : i32,
    p2_start : i32,
    p2_end : i32,
}

fn parse_inputs() -> Vec<Line>
{
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut l = {Line{p1_start:0, p1_end:0, p2_start:0, p2_end:0}};
            let mut split_coma = line.split(",");
            let mut p1 = split_coma.next().expect("BOUH").split("-");
            l.p1_start = p1.next().expect("BOUH").parse::<i32>().unwrap();
            l.p1_end = p1.next().expect("BOUH").parse::<i32>().unwrap();
            let mut p2 = split_coma.next().expect("BOUH").split("-");
            l.p2_start = p2.next().expect("BOUH").parse::<i32>().unwrap();
            l.p2_end = p2.next().expect("BOUH").parse::<i32>().unwrap();
            v.push(l);
        }
    }
    v
}

fn main() {
    let v = parse_inputs();
    let mut nb_incl : i32 = 0;
    let mut nb_overlap : i32 = 0;
    for l in v {
        let p2_in_p1 : bool = (l.p1_start <= l.p2_start) && (l.p1_end >= l.p2_end);
        let p1_in_p2 : bool = (l.p1_start >= l.p2_start) && (l.p1_end <= l.p2_end);
        if p2_in_p1 || p1_in_p2 {
            nb_incl += 1;
        }
        let p1_overlap : bool = (l.p1_start >= l.p2_start) && (l.p1_start <= l.p2_end);
        let p2_overlap : bool = (l.p2_start >= l.p1_start) && (l.p2_start <= l.p1_end);
        if p1_overlap || p2_overlap {
            nb_overlap += 1;
        }
    }
    println!("P1={}", nb_incl);
    println!("P2={}", nb_overlap);
}

