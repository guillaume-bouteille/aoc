use std::fs::File;
use std::io::{self, BufRead};

struct Rucksack {
    compartment_1: String,
    compartment_2: String,
    all: String // Pffffffffffffffffff
}

fn read_inputs(filename: String) -> Result<Vec<Rucksack>, &'static str>{
    if let Ok(file) = File::open(filename) {
        let mut rucksacks = Vec::new();
        for line in io::BufReader::new(file).lines() {
            if let Ok(line_str) = line {
                let half_length = line_str.len() / 2;
                let mut c1 = String::new();
                let mut c2 = String::new();
                if let Some(part) = line_str.get(0..half_length) {
                    c1 = part.to_string();
                }
                if let Some(part) = line_str.get(half_length..) {
                    c2 = part.to_string();
                }
                rucksacks.push(Rucksack{
                    compartment_1:c1,
                    compartment_2:c2,
                    all:line_str
                });
            }
        }
        return Ok(rucksacks);

    } else {
        Err("Failed to open file")
    }
}

fn get_priority(c : char) -> u32 {
    assert!(c.is_ascii_alphabetic());
    match c {
        'a'..='z' => 1+(c as u32)-('a' as u32),
        'A'..='Z' => 27+(c as u32)-('A' as u32),
        _ => 0 // Pfff
    }
}

fn get_unsorted_items(rucksack: &Rucksack) -> Vec<char> {
    let mut unsorted_items = Vec::new();
    for c in rucksack.compartment_1.chars() {
        if let Some(_) = rucksack.compartment_2.find(c) {
            unsorted_items.push(c);
        }
    }
    unsorted_items.sort();
    unsorted_items.dedup();
    return unsorted_items;
}

fn find_badge(group : &[Rucksack]) -> char {
    assert!(group.len() == 3);
    
    for i in group[0].all.chars() {
        let t1 = group[1].all.find(i);
        let t2 = group[2].all.find(i);
        if t1 != None && t2 != None {
            return i
        }
    }
    assert!(false); // wtfbbq!
    'a'
}

fn main() {
    match read_inputs(String::from("input.txt")) {
        Ok(inputs) => {
            // First part
            let mut priority_1 = 0;
            for rucksack in &inputs {
                for item in get_unsorted_items(rucksack) {
                    priority_1 += get_priority(item);
                }
            }
            println!("The first answer is {}", priority_1);
            
            // Second part
            let mut priority_2 = 0;
            // slice rucksack by group of 3
            let nb_groups = inputs.len()/3;
            for i in 0..nb_groups {
                let group = &inputs[i*3..(i+1)*3];
                let c = find_badge(group);
                priority_2 += get_priority(c);
            }
            
            println!("The second answer is {}", priority_2);
            

        },
        Err(e) => {
            println!("Error: {}", e);
        }        
    }
}

