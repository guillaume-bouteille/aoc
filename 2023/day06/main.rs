

use std::io;

fn parse_inputs() -> (Vec<i64>, Vec<i64>, i64, i64) {

    let t_str = io::stdin().lines().next().expect("culcul").unwrap();
    let t = t_str.split_whitespace().skip(1).map(|it| it.parse::<i64>().expect("roh")).collect::<Vec<i64>>();
    let t1 = t_str.split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse::<i64>().expect("rah");

    let d_str = io::stdin().lines().next().expect("culcul").unwrap();
    let d = d_str.split_whitespace().skip(1).map(|it| it.parse::<i64>().expect("roh")).collect::<Vec<i64>>();
    let d1 = d_str.split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse::<i64>().expect("rah");

    (t,d,t1,d1)
}


fn main() {
    let (times, distances, time_1, distance_1) = parse_inputs();
    let mut records_beaten : Vec<i64> = Vec::new();
    for i in 0..times.len() {
        let mut rec : i64 = 0;
        for wait_time in 0..times[i] {
            if wait_time*(times[i]-wait_time) > distances[i] {
                rec += 1;
            }
        }
        records_beaten.push(rec);
    }
    println!("P1={}", records_beaten.iter().fold(1, |acc, it| acc * it));
    let mut record_p2 : i64 = 0;
    for wait_time in 0..time_1 {
        if wait_time*(time_1-wait_time) > distance_1 {
            record_p2 += 1;
        }
    }
    println!("P2={}", record_p2);

}
