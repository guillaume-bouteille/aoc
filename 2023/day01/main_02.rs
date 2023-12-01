
use std::io;

fn get_numbers_in_line(s: String) -> Vec<u32> {

    let mut v = Vec::new();

    for (idx,c) in s.chars().enumerate() {

        if c.is_ascii_digit() {
            v.push(c.to_digit(10).unwrap());
        } else {
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
    //println!("line={} v={:?}", s, v);
    v
}

fn main() {
    let lines = io::stdin().lines();
    let mut sum: u32 = 0;
    for line in lines {
        let v = get_numbers_in_line(line.unwrap());

        let first_digit = v.first().unwrap();
        let last_digit = v.last().unwrap();

        sum = sum + (first_digit*10+last_digit);
    }
    println!("Sum: {}", sum);
}
