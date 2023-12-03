
use std::io;

struct Round {
    opponent_choice : String,
    my_choice : String,
}

fn parse_inputs() -> Vec<Round> {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line_str) = line_it {
            let mut spl = line_str.split(" ");
            v.push(Round{
                opponent_choice:spl.next().unwrap().to_string(),
                my_choice:spl.next().unwrap().to_string()
            });
        }
    }
    v
}

// X for Rock, Y for Paper, and Z for Scissors
// 1 for Rock, 2 for Paper, and 3 for Scissors)
// outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
// A for Rock, B for Paper, and C for Scissors
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win

struct ScoreMapItem
{
    s1: String,
    s2: String,
    res : i32,
}
type ScoreMap = Vec<ScoreMapItem>;

fn get_score(score_map : &ScoreMap, round : &Round) -> i32
{
    // Erf
    for it in score_map {
        if (it.s1 == round.opponent_choice) && (it.s2 == round.my_choice) {
            return it.res;
        }
    }
    assert!(false); // Meuh
    return 0;
}


fn main() {
    let score_map_1 : ScoreMap = Vec::from([
        ScoreMapItem{s1:"A".to_string(),s2:"X".to_string(), res:3+1},
        ScoreMapItem{s1:"A".to_string(),s2:"Y".to_string(), res:6+2},
        ScoreMapItem{s1:"A".to_string(),s2:"Z".to_string(), res:0+3},
        ScoreMapItem{s1:"B".to_string(),s2:"X".to_string(), res:0+1},
        ScoreMapItem{s1:"B".to_string(),s2:"Y".to_string(), res:3+2},
        ScoreMapItem{s1:"B".to_string(),s2:"Z".to_string(), res:6+3},
        ScoreMapItem{s1:"C".to_string(),s2:"X".to_string(), res:6+1},
        ScoreMapItem{s1:"C".to_string(),s2:"Y".to_string(), res:0+2},
        ScoreMapItem{s1:"C".to_string(),s2:"Z".to_string(), res:3+3},
    ]);

    let score_map_2 : ScoreMap = Vec::from([
        ScoreMapItem{s1:"A".to_string(),s2:"X".to_string(), res:0+3},
        ScoreMapItem{s1:"A".to_string(),s2:"Y".to_string(), res:3+1},
        ScoreMapItem{s1:"A".to_string(),s2:"Z".to_string(), res:6+2},
        ScoreMapItem{s1:"B".to_string(),s2:"X".to_string(), res:0+1},
        ScoreMapItem{s1:"B".to_string(),s2:"Y".to_string(), res:3+2},
        ScoreMapItem{s1:"B".to_string(),s2:"Z".to_string(), res:6+3},
        ScoreMapItem{s1:"C".to_string(),s2:"X".to_string(), res:0+2},
        ScoreMapItem{s1:"C".to_string(),s2:"Y".to_string(), res:3+3},
        ScoreMapItem{s1:"C".to_string(),s2:"Z".to_string(), res:6+1},
    ]);

    let rounds = parse_inputs();

    let mut score_1 : i32 = 0;
    for round in &rounds {
        score_1 += get_score(&score_map_1, round);
    }
    println!("P1={}", score_1);

    let mut score_2 : i32 = 0;
    for round in &rounds {
        score_2 += get_score(&score_map_2, round);
    }
    println!("P2={}", score_2);
}

