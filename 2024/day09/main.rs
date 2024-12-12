

use std::io;

type Inputs = Vec<u32>;

fn parse_inputs() -> Inputs {
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            return line.chars().map(|c| c.to_digit(10).expect("blab")).collect::<Vec<u32>>();
        }
    }
    todo!("blah!");
}

fn solve_part_1(inputs: Inputs) -> i64 {
    let mut v = Vec::<i64>::new();
    for (i, value) in inputs.iter().enumerate() {
        let c_to_push = if i % 2 == 0 { (i/2) as i64 } else { -1 };
        for _ in 0..*value {
            v.push(c_to_push);
        }
    }
    //print_disk(&v);
    let mut start_cursor : usize = 0;
    let mut end_cursor : usize = v.len()-1;
    while start_cursor < end_cursor {
        if v[start_cursor] >= 0 {
            start_cursor += 1;
        } else {
            if v[end_cursor] < 0 {
                end_cursor -= 1;
            } else {
                v[start_cursor] = v[end_cursor];
                v[end_cursor] = -1;
            }
        }
    }
    //print_disk(&v);
    let mut res = 0;
    for (i, value) in v.iter().enumerate() {
        if *value >= 0 {
            res += (i as i64)*value;
        }
    }
    res
}

//fn print_disk(v: &Vec<i64>) {
//    for value in v.iter() {
//        if *value < 0 {
//            eprint!(".");
//        } else {
//            eprint!("{}", value % 10);
//        }
//    }
//    eprintln!("");
//}

fn find_free_space(v: &[i64], required_space: i64) -> i64 {
    let mut res = 0;
    let mut count = 0;
    while res < v.len() {
        if v[res] < 0 {
            count += 1;
            if count >= required_space {
                return (res as i64)-count+1;
            }
        } else {
            count = 0;
        }
        res += 1;
    }
    -1
}

fn solve_part_2(inputs: Inputs) -> i64 {
    let mut v = Vec::<i64>::new();
    for (i, value) in inputs.iter().enumerate() {
        let c_to_push = if i % 2 == 0 { (i/2) as i64 } else { -1 };
        for _ in 0..*value {
            v.push(c_to_push);
        }
    }
    //print_disk(&v);
    let mut end_cursor : i64 = (v.len()-1) as i64;
    while end_cursor > 0 {
        if v[end_cursor as usize] < 0 {
            end_cursor -= 1;
        } else {
            let mut start_end_cursor = end_cursor;
            while start_end_cursor > 0 && v[start_end_cursor as usize] == v[end_cursor as usize] {
                start_end_cursor -= 1;
            }
            let free_space_cursor = find_free_space(&v[0..((start_end_cursor+1) as usize)], end_cursor-start_end_cursor);
            if free_space_cursor > 0 {
                for i in 0..(end_cursor-start_end_cursor) {
                    v[(free_space_cursor+i) as usize] = v[(start_end_cursor+1+i) as usize];
                    v[(start_end_cursor+1+i) as usize] = -1;
                }
            }
            end_cursor = start_end_cursor;
        }
    }

    //print_disk(&v);
    let mut res = 0;
    for (i, value) in v.iter().enumerate() {
        if *value >= 0 {
            res += (i as i64)*value;
        }
    }
    res
}

fn main() {

    let inputs = parse_inputs();

    let p1 = solve_part_1(inputs.clone());
    let p2 = solve_part_2(inputs);

    println!("P1={}", p1);
    println!("P2={}", p2);
}
