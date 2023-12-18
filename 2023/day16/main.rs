

use std::io;
use std::collections::HashSet;
use std::cmp;

type Inputs = Vec<Vec<char>>;

fn parse_inputs() -> Inputs {
    let mut v : Inputs = Vec::new();
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            v.push( line.chars().collect::<Vec<char>>() );
        }
    }
    v
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Ray {
    x : i64,
    y : i64,
    dir : char,
}


fn trace_ray(inputs: &Inputs, ray: Ray) -> i64 {
    let mut energy_map = vec![ vec![ false; inputs[0].len() ]; inputs.len() ];
    let mut rays : Vec<Ray> = Vec::new();
    rays.push(ray);

    let mut seen_places : HashSet<Ray> = HashSet::new();
    while rays.len() > 0 {

        let mut r = &mut rays[0];
        let is_in_bound = r.y >= 0 && r.y < inputs.len() as i64 && r.x >= 0 && r.x < inputs[0].len() as i64;
        if is_in_bound == false {
            rays.remove(0);
            continue;
        }
        if seen_places.contains(r) {
            rays.remove(0);
            continue;
        }
        seen_places.insert(r.clone());
        energy_map[r.y as usize][r.x as usize] = true;
        let mut new_ray = Ray{x:-1, y:-1,dir:'~'};
        let tile = inputs[r.y as usize][r.x as usize];
        match (tile,r.dir) {
            ('/','>') => {
                r.y -=1;
                r.dir = '^';
            },
            ('/', '<') => {
                r.y += 1;
                r.dir = 'v';
            },
            ('/', '^') => {
                r.x += 1;
                r.dir = '>'
            },
            ('/', 'v') => {
                r.x -= 1;
                r.dir = '<'
            },
            ('\\', '>') => {
                r.y +=1;
                r.dir = 'v';
            },
            ('\\','<') => {
                r.y -=1;
                r.dir = '^';
            },
            ('\\','^') => {
                r.x -=1;
                r.dir = '<';
            },
            ('\\','v') => {
                r.x +=1;
                r.dir = '>';
            },
            ('|','>')|('|','<') => {
                new_ray = Ray{x:r.x, y:r.y+1, dir:'v'};
                r.y -= 1;
                r.dir = '^';
            },
            ('|','^')|('.','^') => {
                r.y -= 1;
            },
            ('|','v')|('.','v') => {
                r.y += 1;
            },
            ('-','^')|('-','v') => {
                new_ray = Ray{x:r.x+1, y:r.y, dir:'>'};
                r.x -= 1;
                r.dir = '<';
            },
            ('-','>')|('.','>') => {
                r.x += 1;
            },
            ('-','<')|('.','<') => {
                r.x -= 1;
            },
            _ => todo!()
        }
        if new_ray.dir != '~' {
            rays.push(new_ray);
        }
    }
   
    let mut energy = 0;
    for l in energy_map {
        for t in l {
            if t {
                energy += 1;
            }
        }
    }
    energy
}

fn main() {

    let inputs = parse_inputs();

    let p1 = trace_ray(&inputs, Ray{x:0,y:0,dir:'>'});
    println!("P1={}", p1);

    let mut p2 : i64 = 0;
    for y in 0..inputs.len() {
        p2 = cmp::max(p2, trace_ray(&inputs, Ray{x:0,                         y:y as i64,dir:'>'}));
        p2 = cmp::max(p2, trace_ray(&inputs, Ray{x:(inputs[0].len()-1) as i64,y:y as i64,dir:'<'}));
    }
    for x in 0..inputs[0].len() {
        p2 = cmp::max(p2, trace_ray(&inputs, Ray{x:x as i64,y:0,                      dir:'v'}));
        p2 = cmp::max(p2, trace_ray(&inputs, Ray{x:x as i64,y:(inputs.len()-1) as i64,dir:'^'}));
    }
    println!("P2={}", p2);
}
