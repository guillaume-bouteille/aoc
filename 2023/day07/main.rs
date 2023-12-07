
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;


#[derive(Clone)]
#[allow(dead_code)] // raw_hand used for debug
struct Hand {
    raw_hand : Vec<char>,
    hand_type : i32,
    hand_as_ints : Vec<i32>
}

type Bid = i64;

fn parse_inputs() -> Vec<(Hand, Hand, Bid)> {
    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut spl = line.split(" ");
            let raw_hand : Vec<char> = spl.next().expect("hihihi").chars().collect();
            let hand = {Hand{
                raw_hand: raw_hand.clone(),
                hand_type: get_hand_type(&raw_hand),
                hand_as_ints: raw_hand.iter().map(|card| card_to_int(card)).collect::<Vec<i32>>(),
            }};
            let hand_p2 = {Hand{
                raw_hand: raw_hand.clone(),
                hand_type: get_hand_type_p2(&raw_hand),
                hand_as_ints: raw_hand.iter().map(|card| card_to_int_p2(card)).collect::<Vec<i32>>(),
            }};
            v.push((
                hand,
                hand_p2,
                spl.next().expect("hohoho").parse::<i64>().expect("ohlala")
            ));
        }
    }
    v
}

const HIGH_CARD : i32 = 0;
const ONE_PAIR : i32 = 1;
const TWO_PAIRS : i32 = 2;
const THREE_OF_A_KIND : i32 = 3;
const FULL_HOUSE : i32 = 4;
const FOUR_OF_A_KIND : i32 = 5;
const FIVE_OF_A_KIND : i32 = 6;

fn get_hand_type_p2(rh: &Vec<char>) -> i32 {
    let nb_j = rh.iter().filter(|card| **card == 'J').count();
    if (nb_j == 5) || (nb_j == 4) {
        return FIVE_OF_A_KIND;
    }
    let mut best = get_hand_type(rh);
    let mut rh_clone = rh.clone();
    for (i,c) in rh.iter().enumerate() {
        if *c == 'J' {
            for t in vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'] {
                rh_clone[i] = t;
                best = std::cmp::max(best, get_hand_type_p2(&rh_clone));
            }
        }
    }
    best
}

fn get_hand_type(rh: &Vec<char>) -> i32 {
    let mut h = HashMap::new();
    for card in rh {
        h.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
    }
    if h.values().any(|v| *v == 5) {
        return FIVE_OF_A_KIND;
    } else if h.values().any(|v| *v == 4) {
        return FOUR_OF_A_KIND;
    } else if h.values().find(|v| **v == 2).is_some() && h.values().find(|v| **v == 3).is_some() {
        return FULL_HOUSE;
    } else if h.values().any(|v| *v == 3) {
        return THREE_OF_A_KIND;
    } else if h.values().filter(|v| **v == 2).count() == 2 {
        return TWO_PAIRS;
    } else if h.values().filter(|v| **v == 2).count() == 1 {
        return ONE_PAIR;
    } else {
        return HIGH_CARD;
    }
}

fn card_to_int(rh: &char) -> i32 {
    match rh {
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'J' => 11,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
    _ => todo!() 
    }
}

fn card_to_int_p2(rh: &char) -> i32 {
    match rh {
    'J' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
    _ => todo!() 
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {

    if a.hand_type != b.hand_type {
        a.hand_type.cmp(&b.hand_type)
    } else {
        for i in 0..a.hand_as_ints.len() {
            if a.hand_as_ints[i] != b.hand_as_ints[i] {
                return a.hand_as_ints[i].cmp(&b.hand_as_ints[i]);
            }
        }
        Ordering::Equal
    }
}

fn main() {
    let inputs = parse_inputs();

    let mut sorted_hands = inputs.clone();
    sorted_hands.sort_by(|a,b| compare_hands(&a.0,&b.0));
    let score_p1 = sorted_hands.iter().enumerate()
        .fold(0, |acc, (i,gamer)| acc + ((i+1) as i64)*gamer.2);
    println!("P1={}", score_p1);

    let mut sorted_hands_p2 = inputs.clone();
    sorted_hands_p2.sort_by(|a,b| compare_hands(&a.1,&b.1));
    let score_p2 = sorted_hands_p2.iter().enumerate()
        .fold(0, |acc, (i,gamer)| acc + ((i+1) as i64)*gamer.2);
    println!("P2={}", score_p2);
}
