
use std::io;

fn get_numbers_in_line(s: &String, allow_text : bool) -> Vec<u32> {
    let mut v = Vec::new();

    for (idx,c) in s.chars().enumerate() {

        if c.is_ascii_digit() {
            v.push(c.to_digit(10).unwrap());
        } else if allow_text {
            let substr = s.get(idx..).unwrap();
            if substr.starts_with("one") {
                v.push(1);
            }
            if substr.starts_with("two") {
                v.push(2);
            }
            if substr.starts_with("three") {
                v.push(3);
            }
            if substr.starts_with("four") {
                v.push(4);
            }
            if substr.starts_with("five") {
                v.push(5);
            }
            if substr.starts_with("six") {
                v.push(6);
            }
            if substr.starts_with("seven") {
                v.push(7);
            }
            if substr.starts_with("eight") {
                v.push(8);
            }
            if substr.starts_with("nine") {
                v.push(9);
            }
        }
    }
    v
}

fn main() {
    let mut sum_p1: u32 = 0;
    let mut sum_p2: u32 = 0;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {

            let v1 = get_numbers_in_line(&line, false);
            sum_p1 += v1.first().unwrap()*10 + v1.last().unwrap();

            let v2 = get_numbers_in_line(&line, true);
            sum_p2 += v2.first().unwrap()*10 + v2.last().unwrap();
        }
    }
    println!("P1={}", sum_p1);
    println!("P2={}", sum_p2);
}
