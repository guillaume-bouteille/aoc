

use std::io;

#[derive(Clone)]
struct Inputs {
    order: Vec<(i64,i64)>,
    updates: Vec<Vec<i64>>,
}


fn parse_inputs() -> Inputs {
    let mut inputs : Inputs = Inputs{order: Vec::new(), updates: Vec::new()};
    let mut is_order = true;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if is_order {
                let t = line.split("|").collect::<Vec<&str>>();
                if t.len() == 2 {
                    inputs.order.push((t[0].parse::<i64>().expect("tutu"),t[1].parse::<i64>().expect("titi")));
                } else {
                    is_order = false;
                }
            } else {
                let t = line.split(",").map(|e| e.parse::<i64>().expect("tonton") ).collect::<Vec<i64>>();
                inputs.updates.push(t)
            }
        }
    }
    inputs
}

fn get_wrong_order(update: &Vec<i64>, orders: &Vec<(i64,i64)>) -> Option<(usize,usize)> {
    for order in orders {

        let idx1 = update.iter().position(|e| *e == order.0 );
        let idx2 = update.iter().position(|e| *e == order.1 );
        if let Some(idx1) = idx1 {
            if let Some(idx2) = idx2 {
                if idx1 > idx2 {
                    return Some((idx1,idx2));
                }
            }    
        }
    }
    return None
}

fn solve_part_1(inputs: Inputs) -> i64 {
    let mut res = 0;
    for update in &inputs.updates {
        let wo = get_wrong_order(update, &inputs.order);
        if wo.is_none() {
            res += update[update.len()/2];
        }
    }
    res
}

fn solve_part_2(mut inputs: Inputs) -> i64 {
    let mut res = 0;
    for update in &mut inputs.updates {
        if get_wrong_order(update, &inputs.order).is_none() {
            continue;
        }
        loop {
            let wo = get_wrong_order(update, &inputs.order);
            if let Some(wo) = wo {
                let v = update[wo.0];
                update[wo.0] = update[wo.1];
                update[wo.1] = v
            } else {
                break;
            }
        }
        res += update[update.len()/2];
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
