
use std::io;


fn parse_inputs() -> Vec<i32> {
    let mut v : Vec<i32> = Vec::new();
    let mut calo : i32 = 0;
    for line_it in io::stdin().lines() {
        if let Ok(line_str) = line_it {
            if line_str.len() > 0 {
                calo += line_str.parse::<i32>().unwrap();
            } else {
                v.push(calo);
                calo = 0;
            }
        }
    }
    v
}
fn main() {
    let mut v = parse_inputs();
    v.sort();
    println!("P1={}", v.last().unwrap());
    println!("P2={}", v.get(v.len()-3..).unwrap().iter().sum::<i32>());
}
