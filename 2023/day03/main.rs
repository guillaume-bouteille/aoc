
use std::io;

#[derive(Clone)]
#[derive(Copy)]
struct Number {
    value: u32,
    start_x: i32,
    end_x: i32,
}

#[allow(dead_code)] // symbol used only for debug
struct Engine {
    symbol: char,
    x: i32,
}

struct Line {
    numbers: Vec<Number>,
    engines: Vec<Engine>,
}

struct Game {
    lines: Vec<Line>,
}

fn parse_inputs() -> Game {
    let mut game = {Game{lines:Vec::new()}};
    for (_y, line_it) in io::stdin().lines().enumerate() {
        if let Ok(line) = line_it {
            let mut line_obj = {Line{numbers:Vec::new(), engines:Vec::new()}};

            let mut is_in_number = false;
            let mut tmpnumber = Number{value:0, start_x:0, end_x:0}; 
            for (x, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    if is_in_number == false {
                        tmpnumber.start_x = x as i32;
                    }
                    is_in_number = true;
                    tmpnumber.value = tmpnumber.value*10 + c.to_digit(10).unwrap();
                    tmpnumber.end_x = x as i32;

                    if x+1 == line.len() { // The line ends with a digit
                        line_obj.numbers.push(tmpnumber);
                        tmpnumber = Number{value:0, start_x:0, end_x:0}; 
                    }
                } else {
                    if is_in_number == true {
                        is_in_number = false;
                        line_obj.numbers.push(tmpnumber);
                        tmpnumber = Number{value:0, start_x:0, end_x:0}; 
                    }

                    if c != '.' { // Engine
                        line_obj.engines.push(Engine{symbol:c, x:x as i32});
                    }
                }
            }

            game.lines.push(line_obj);
        }
    }
    game
}

fn main() {
    let game = parse_inputs();

    // Solve P1
    let mut sum_adjacent : u32 = 0;
    for (line_y, line_num) in game.lines.iter().enumerate() {
        // TODO: maybe do something nice with .windows(3)?
        let mut min_y = line_y;
        if line_y > 0 {
            min_y = line_y-1;
        }
        let mut max_y = line_y;
        if line_y+1 < game.lines.len() {
            max_y = line_y+1;
        }
        for num in &line_num.numbers {
            let mut is_adjacent : bool = false;
//            println!("Number {} at {},{}", num.value, num.start_x, line_num.y);
            for line_engine in game.lines.get(min_y..max_y+1).expect("Window empty") {
                for engine in &line_engine.engines {
//                        println!("Engine {} at {},{}", engine.symbol, engine.x, line_engine.y);
                    if (engine.x >= (num.start_x-1)) && (engine.x <= (num.end_x+1)) {
                        is_adjacent = true;
//                            println!("Engine {} at {},{} adjacent to number {} at {},{}", engine.symbol, engine.x, line_engine.y, num.value, num.start_x, line_num.y);
                    }
                }
            }

            if is_adjacent {
                sum_adjacent += num.value;
            }
        }
    }
    println!("P1={}", sum_adjacent);

    // Solve P2
    let mut sum_gear_ratio : u32 = 0;
    for (line_y, line_engine) in game.lines.iter().enumerate() {
        let mut min_y = line_y;
        if line_y > 0 {
            min_y = line_y-1;
        }
        let mut max_y = line_y;
        if line_y+1 < game.lines.len() {
            max_y = line_y+1;
        }
        for engine in &line_engine.engines {
            let mut adjacent_num : Vec<Number> = Vec::new();
            for line_num in game.lines.get(min_y..max_y+1).expect("Window empty") {
                for num in &line_num.numbers {
                    if (engine.x >= (num.start_x-1)) && (engine.x <= (num.end_x+1)) {
                        adjacent_num.push(*num);
                    }
                }
            }
            if adjacent_num.len() == 2 {
                sum_gear_ratio += adjacent_num[0].value * adjacent_num[1].value;
            }

        }
    }
    println!("P2={}", sum_gear_ratio);
}
