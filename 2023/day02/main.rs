
use std::io;

struct Game {
    index : u32,
    turns : Vec<GameTurn>,
}

struct GameTurn {
    red : u32,
    blue : u32,
    green : u32,
}

fn main() {
    let mut games = Vec::new();
    // Parse inputs
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut game : Game = { Game{index:0, turns:Vec::new()} };
            let mut line_split = line.split(": ");

            game.index = line_split.next().unwrap().get(5..).unwrap().parse::<u32>().unwrap();
//            println!("got a line: {}", game.index);
            for turn in line_split.next().unwrap().split("; ") {
                let mut gt = { GameTurn{red:0, blue:0, green:0} };
                for color in turn.split(", ") {
                    if let Some(idx) = color.find(" red") {
                        gt.red = color.get(..idx).unwrap().parse::<u32>().unwrap();
                    }
                    if let Some(idx) = color.find(" blue") {
                        gt.blue = color.get(..idx).unwrap().parse::<u32>().unwrap();
                    }
                    if let Some(idx) = color.find(" green") {
                        gt.green = color.get(..idx).unwrap().parse::<u32>().unwrap();
                    }
                }
                game.turns.push(gt);
            }
            
            games.push(game);
        }
    }
    // Solve p1
    let mut sum : u32 = 0;
    for game in &games {
        let max_red = game.turns.iter().map(|t| t.red).max().unwrap_or(0);
        let max_green = game.turns.iter().map(|t| t.green).max().unwrap_or(0);
        let max_blue = game.turns.iter().map(|t| t.blue).max().unwrap_or(0);
        if (max_red <= 12) && (max_green <= 13) && (max_blue <= 14) {
            sum += game.index;
        }
    }
    println!("Result P1 is {}", sum);
    let mut sum_2 : u32 = 0;
    for game in &games {
        let max_red = game.turns.iter().map(|t| t.red).max().unwrap_or(0);
        let max_green = game.turns.iter().map(|t| t.green).max().unwrap_or(0);
        let max_blue = game.turns.iter().map(|t| t.blue).max().unwrap_or(0);
        sum_2 += max_red * max_green * max_blue;
    }
    println!("Result P2 is {}", sum_2);
}
