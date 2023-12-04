
use std::io;

struct Card {
    winning_numbers : Vec<i32>,
    my_numbers : Vec<i32>
}

fn parse_inputs() -> Vec<Card> {

    let mut v = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            let mut card = Card{winning_numbers:Vec::new(), my_numbers:Vec::new()};

            let mut l_split = line.split(": ").last().unwrap().split("|");

            card.winning_numbers = l_split.next().unwrap().trim().split(" ")
                .filter(|the_str| the_str.len() > 0)
                .map(|int_str| { int_str.parse::<i32>().unwrap() }).collect();
            card.my_numbers = l_split.next().unwrap().trim().split(" ")
                .filter(|the_str| the_str.len() > 0)
                .map(|int_str| int_str.parse::<i32>().unwrap()).collect();
            v.push(card);
        }
    }
    v
}

fn main() {

    let cards = parse_inputs();

    let mut score : i32 = 0;
    for card in &cards {
        let s = card.my_numbers.iter().filter(|n| card.winning_numbers.contains(n)).count();
        if s > 0 {
            score += 1 << (s-1);
        }
    }
    let mut nb_scratch_cards : Vec<i32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let s = card.my_numbers.iter().filter(|n| card.winning_numbers.contains(n)).count();
        if s > 0 {
            for j in 1..(s+1) {
                if i+j < nb_scratch_cards.len() {
                    nb_scratch_cards[i+j] += nb_scratch_cards[i];
                }
            }
        }
    }
    println!("P1={}", score);
    println!("P2={}", nb_scratch_cards.iter().sum::<i32>());
}
