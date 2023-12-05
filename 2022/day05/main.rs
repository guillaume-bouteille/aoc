
use std::io;


type Crates = Vec<Vec<char>>;
struct Move {
    nb : i32,
    from : usize,
    to : usize
}
type Moves = Vec<Move>;

fn parse_inputs() -> (Crates, Moves) {
    let mut crates : Crates = Vec::new();
    let mut moves : Moves = Vec::new();
    let mut end_of_crates = false;
    let mut crates_lines : Vec<String> = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if end_of_crates == false {
                if line.len() == 0 {
                    end_of_crates = true;
                } else {
                    crates_lines.push(line);
                }
            } else {
                let mut parsed_move = {Move{nb:0, from:0, to:0}};
                let (the_move, after) = line.split_once(" from ").expect("BOUH!");
                parsed_move.nb = the_move.get(5..).expect("BOUH!").parse::<i32>().unwrap();
                let (from, to) = after.split_once(" to ").expect("BOUH!");
                parsed_move.from = from.parse::<usize>().unwrap();
                parsed_move.to = to.parse::<usize>().unwrap();
                moves.push(parsed_move);
            }
        }
    }
    let nb_crates = (crates_lines.last().expect("BOUH!").len()+1)/4;
    for _i in 1..nb_crates+1 {
        crates.push(Vec::new());
    }
    for cl in crates_lines.get(..crates_lines.len()-1).expect("BOUH").iter().rev() {
        for i in 0..nb_crates {
            if cl.chars().nth(i*4+1).expect("BOUH") != ' ' {
                crates[i].push(cl.chars().nth(i*4+1).expect("BOUH"));
            }
        }
    }
    (crates, moves)
}

fn main() {
    let (crates,moves) = parse_inputs();

    let mut crates_1 = crates.clone();
    let mut crates_2 = crates;
    for mov in &moves {
        for _i in 0..mov.nb {
            let t = crates_1[mov.from-1].pop().expect("BOUH");
            crates_1[mov.to-1].push(t);
        }
    }
    println!("P1={}", crates_1.iter().map(|c| c.last().expect("BOUH")).collect::<String>());
    for mov in moves {
        let mut t2 = Vec::new();
        for _i in 0..mov.nb {
            let t = crates_2[mov.from-1].pop().expect("BOUH");
            t2.push(t);
        }
        for t in t2.iter().rev() {
            crates_2[mov.to-1].push(*t);
        }
    }
    println!("P2={}", crates_2.iter().map(|c| c.last().expect("BOUH")).collect::<String>());
}
