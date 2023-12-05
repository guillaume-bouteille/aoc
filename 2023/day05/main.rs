
use std::io;

type Seeds = Vec<usize>;

struct ResourceMap {
    destination: usize,
    source: usize,
    length : usize,
}

fn parse_inputs() -> (Seeds, Vec<Vec<ResourceMap>>) {
    let mut seeds = Vec::new();
    let mut conv = Vec::new();

    let mut state : i32 = 0;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if line.len() == 0 {
                state += 1;
                match state {
                    _ => { conv.push(Vec::new());}
                }
            } else {
                match state {
                    0 => { // Seeds
                        let (_, raw_seeds) = line.split_once(": ").expect("GRRR");
                        seeds = raw_seeds.split(" ").map(|it| it.parse::<usize>().expect("YOH")).collect();
                        }
                    _ => {
                        if line.chars().next().expect("truc").is_digit(10) {
                            let mut ls = line.split(" ");
                            let r = ResourceMap{
                                    destination: ls.next().expect("arsit").parse::<usize>().unwrap(),
                                    source:ls.next().expect("rairset").parse::<usize>().unwrap(),
                                    length:ls.next().expect("tututut").parse::<usize>().unwrap()
                            };
                            conv.last_mut().expect("blah").push(r);
                        }
                    }
                }
            }
        }
    }
    (seeds, conv)
}

fn get_new_id(prev_id: &mut usize, conv: &Vec<ResourceMap>) {
    for rm in conv {
        if (*prev_id >= rm.source) && (*prev_id <= (rm.source + rm.length)) {
            *prev_id = rm.destination + (*prev_id - rm.source);
            return;
        }
    }
    // No modification of prev_id
}

fn get_new_id_for_range(prev_range_start : usize, prev_range_len : usize, conv: &Vec<ResourceMap>)
    -> Vec<(usize,usize)> {
    let mut result = Vec::new();
    let mut remaining = Vec::new();
    remaining.push((prev_range_start, prev_range_len));
    for rm in conv {
        let mut new_remaining = Vec::new();
        for (start, length) in remaining {
            if (start+length) < rm.source {
                new_remaining.push((start, length));
            } else if start > rm.source+rm.length {
                new_remaining.push((start, length));
            } else if (start >= rm.source) && ((start+length) <= rm.source + rm.length) {
                result.push( (rm.destination + (start - rm.source), length));
            } else if (start <= rm.source) && ((start+length) >= rm.source + rm.length) {
                new_remaining.push((start, rm.source-start));
                result.push((rm.destination, rm.length));
                new_remaining.push((rm.source+rm.length, (start+length)-(rm.source+rm.length)));
            } else if (start <= rm.source) && ((start+length) < rm.source + rm.length) {
                //eprintln!("COIN {} {} --- {} {}", start, start+length, rm.source, rm.source+rm.length);
                new_remaining.push((start, rm.source-start));
                result.push((rm.destination, (rm.source+rm.length)-(start+length)));
            } else if (start > rm.source) && ((start+length) >= rm.source + rm.length) {
                result.push((rm.destination + (start - rm.source), (rm.source+rm.length)-start));
                new_remaining.push((rm.source+rm.length, (start+length)-(rm.source+rm.length)));
            } else {
                eprintln!("BOU {} {} --- {} {}", start, length, rm.source, rm.length);
            }
        }
        remaining = new_remaining;
    }
    result.extend(remaining);
    result
}

fn main() {
    let (seeds, conversions) = parse_inputs();
    let mut p1_seeds = seeds.clone();
    for conv in &conversions {
        for s in &mut p1_seeds {
            get_new_id(s, &conv);
        }
    }
    println!("P1={}", p1_seeds.iter().min().expect("kikoo"));
    let mut p2_seed_ranges : Vec<(usize,usize)> = seeds.chunks(2).map(|it| (it[0], it[1])).collect();
    //for (s, l) in &p2_seed_ranges {
        //eprintln!("{} -> {}", s, s+l);
    //}
    for conv in conversions {
        let mut new_ranges : Vec<(usize, usize)> = Vec::new();
        for (start, length) in p2_seed_ranges {
            new_ranges.extend(
                get_new_id_for_range(start, length, &conv));
        }
        p2_seed_ranges = new_ranges;
        //eprintln!("-------------------");
        //for (s, l) in &p2_seed_ranges {
            //eprintln!("{} -> {}", s, s+l);
        //}
    }
    println!("P2={}", p2_seed_ranges.iter().map(|(start,_)| start).min().expect("ohla"));

}
