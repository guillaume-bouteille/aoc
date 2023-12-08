
use std::io;
use std::collections::HashMap;

type Inputs = (Vec<char>,HashMap<String,(String,String)>);

fn parse_inputs() -> Inputs {
    let mut instructions = Vec::new();
    let mut map = HashMap::new();
    let mut line_cnt = 0;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            line_cnt += 1;
            match line_cnt {
                1 => instructions = line.chars().collect(),
                2 => {},
                _ => {
                    let it = line.split_once(" = (");
                    let (orig, tmp) = it.expect("wouhou");
                    let (l, r) = tmp.split_once(", ").expect("kikoo");
                    map.insert(
                        orig.to_string(),
                        (l.to_string(), r.get(..r.len()-1).expect("narusiet").to_string()));
                }
            }
        }
    }
    (instructions, map)
}

fn main() {
    let (instructions, map) = parse_inputs();

    let mut pos = "AAA".to_string();
    let mut cnt : usize = 0; 
    while pos != "ZZZ" {
        let next_instruction = instructions[cnt%instructions.len()];
        if next_instruction == 'L' {
            pos = map.get(&pos).expect("ttttt").0.clone();
        } else {
            pos = map.get(&pos).expect("zzzzz").1.clone();
        }
        cnt += 1;
    }
    println!("P1={}", cnt);

    let mut positions = map.iter().filter(|(k,_)| k.ends_with("A")).map(|(k,_)| k.clone())
        .collect::<Vec<String>>();
    let mut cnt_p2 = vec![0 as usize; positions.len()];
    for (i, pos) in positions.iter_mut().enumerate() {
        let mut cnt_x = 0;
        while pos.ends_with("Z") == false {
            let next_instruction = instructions[cnt_x%instructions.len()];
            if next_instruction == 'L' {
                *pos = map.get(&pos.clone()).expect("ttttt").0.clone();
            } else {
                *pos = map.get(&pos.clone()).expect("zzzzz").1.clone();
            }
            cnt_x += 1;
        }
        cnt_p2[i] = cnt_x;
    }
    let res_p2 = cnt_p2.clone().into_iter().reduce(|acc, e| {
        let mut pgcd = acc;
        let mut m = e;
        while m != 0 {
          if m < pgcd {
            std::mem::swap(&mut m, &mut pgcd);
          }
          m %= pgcd;
        }
        // PPCM is product divided by PGCD
        (acc*e)/pgcd
    }).expect("brah");
    println!("P2={}", res_p2);
}
