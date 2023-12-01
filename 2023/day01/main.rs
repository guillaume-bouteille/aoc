
use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut sum: u32 = 0;
    for line in lines {
        let mut first_found: bool = false;
        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        //println!("got a line: {}", line.unwrap());

        for c in line.unwrap().chars() {
            //println!("got a char: {}", c);
            if c.is_ascii_digit()  {
                if first_found == false {
                    first_found = true;
                    first_digit = c.to_digit(10).unwrap();
                }

                last_digit = c.to_digit(10).unwrap();
            }

        }

        sum = sum + (first_digit*10+last_digit);
    }
    println!("Sum: {}", sum);
}
