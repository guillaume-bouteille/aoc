

use std::io;

fn parse_inputs() -> (Vec<i64>, Vec<i64>, i64, i64) {

    let t_str = io::stdin().lines().next().expect("culcul").unwrap();
    let t = t_str.split_whitespace().skip(1).map(|it| it.parse::<i64>().expect("roh"))
        .collect::<Vec<i64>>();
    let t1 = t_str.split_whitespace().skip(1).collect::<Vec<&str>>().join("")
        .parse::<i64>().expect("rah");

    let d_str = io::stdin().lines().next().expect("culcul").unwrap();
    let d = d_str.split_whitespace().skip(1).map(|it| it.parse::<i64>().expect("roh"))
        .collect::<Vec<i64>>();
    let d1 = d_str.split_whitespace().skip(1).collect::<Vec<&str>>().join("")
        .parse::<i64>().expect("rah");

    (t,d,t1,d1)
}

fn get_nb_records_beaten(time: i64, distance: i64) -> i64 {

    // back to school y = a*x*x + b*x + c, with
    // a = -1
    // b = time
    // c = -distance
    let discriminant : i64 = time*time-(4*(-1)*(-distance));

    assert!(discriminant > 0); // Faut pas déconner, non plus

    // According to some guy on a forum, loss of precision of sqrt is under 1 digit for all
    // integers less than 2**52
    assert!(discriminant < ((1 as i64) << 52));
    let sqrt_disc : i64 = (discriminant as f64).sqrt() as i64;
    // Sanity check
    assert!(sqrt_disc*sqrt_disc < discriminant);
    assert!((sqrt_disc+1)*(sqrt_disc+1) > discriminant);

    // Distance of 2 solutions is sqrt(discriminant) / a
    let s1 : i64 = (((-time) + sqrt_disc)) / (-2);
    let s2 : i64 = (((-time) - sqrt_disc)) / (-2);
    let mut s_min = std::cmp::min(s1, s2);
    let mut s_max = std::cmp::max(s1, s2);

    // Check a few values around the zero crossing solutions
    for _ in 0..3 {
        if s_min*(time-s_min) > distance {
            s_min -=1;
        }
        if s_max*(time-s_max) > distance {
            s_max +=1;
        }
    }
    ((s_max-1)-(s_min+1)) + 1

//    let mut rec : i64 = 0;
//    for wait_time in 0..time {
//        if wait_time*(time-wait_time) > distance {
//            rec += 1;
//        }
//    }
//    rec
}

fn main() {
    let (times, distances, time_1, distance_1) = parse_inputs();
    let record_p1 : i64 = times.iter().enumerate()
        .map(|(i,t)| get_nb_records_beaten(*t, distances[i]))
        .fold(1, |acc, it| acc * it);
    println!("P1={}", record_p1);
    let record_p2 : i64 = get_nb_records_beaten(time_1, distance_1);
    println!("P2={}", record_p2);

}
