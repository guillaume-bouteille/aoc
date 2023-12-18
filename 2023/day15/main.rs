

use std::io;

type Inputs = Vec<String>;

fn parse_inputs() -> Inputs {
    let mut v : Inputs = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.extend( line.split(",").map(|e| e.to_string()).collect::<Vec<String>>() );
        }
    }
    v
}

fn compute_hash(s : &str) -> i64 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as i64;
        current_value *= 17;
        current_value = current_value % 256;
    }
    current_value
}

fn main() {

    let inputs = parse_inputs();

    let mut p1 : i64 = 0;
    for s in &inputs {
        p1 += compute_hash(&s);    
    }
    println!("P1={}", p1);

    let mut p2 : i64 = 0;
    let mut boxes : Vec<Vec<(&str, i64)>> = vec![ Vec::new(); 256 ];
    for s in inputs.iter() {
        if s.chars().last().expect("blah") == '-' {
            let instruction_label = &s[..s.len()-1];
            let h = compute_hash(instruction_label) as usize;
            boxes[h].retain(|(label,_)| *label != instruction_label);
        } else {
            assert!(s.contains('='));
            let (instruction_label, focal_length_str) = s.split_once('=').expect("kikoo");
            let h = compute_hash(instruction_label) as usize;
            let focal_length = focal_length_str.parse::<i64>().unwrap();
            let t = boxes[h].iter_mut().find(|(label,_)| *label == instruction_label);
            if let Some(it) = t {
                it.1 = focal_length;
            } else {
                boxes[h].push((instruction_label, focal_length));
            }
        }
    }
    for (box_idx, the_box) in boxes.iter().enumerate() {
        for (slot_idx, lens) in the_box.iter().enumerate() {
            p2 += (((box_idx+1) * (slot_idx+1)) as i64) * lens.1;
        }
    }
    println!("P2={}", p2);
}
