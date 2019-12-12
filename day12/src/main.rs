use std::fs;

use std::collections::HashMap;

#[macro_use] extern crate lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
struct Vector(i32, i32, i32);

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
struct Orbit {
    position: Vector,
    velocity: Vector
}

impl Orbit {
    pub fn new(input: &str) -> Orbit {
        lazy_static! {
            static ref PARSE_COORDINATES: Regex = Regex::new(r"<x=(-?\d*), y=(-?\d*), z=(-?\d*)>").unwrap();
        }

        for cap in PARSE_COORDINATES.captures_iter(input) {
            return Orbit { position: Vector(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()), velocity: Vector(0, 0, 0) };
        }
        panic!("{} is not an orbit", input);
    }
}

fn offset(a: i32, b: i32) -> i32 {
    return if a > b {-1} else if a < b {1} else {0};
}

fn apply_step(orbits: &Vec<Orbit>, orbit: &Orbit) -> Orbit {
    let mut position = orbit.position;
    let mut velocity = orbit.velocity;
    for o in orbits.iter() {
        velocity.0 += offset(orbit.position.0, o.position.0);
        velocity.1 += offset(orbit.position.1, o.position.1);
        velocity.2 += offset(orbit.position.2, o.position.2);
    }
    position.0 += velocity.0;
    position.1 += velocity.1;
    position.2 += velocity.2;

    return Orbit { position: position, velocity: velocity };
}

fn step(orbits: Vec<Orbit>) -> Vec<Orbit> {
    return orbits.iter().map(|o| apply_step(&orbits, o)).collect();
}

fn sum_abs_l2(v: Vector) -> i32 {
    return v.0.abs() + v.1.abs() + v.2.abs();
}

fn energy(orbits: &Vec<Orbit>) -> i32 {
    return orbits.iter().map(|o| sum_abs_l2(o.position) * sum_abs_l2(o.velocity)).sum::<i32>();
}

fn cycle_length(orbits: Vec<Orbit>) -> i32 {
    let mut previous = HashMap::<Vec<Orbit>, i32>::new();
    previous.insert(orbits.clone(), 0);
    let mut o = orbits;
    let mut steps = 0;
    loop {
        o = step(o);
        steps += 1;
        let prior = previous.get(&o);
        if let Some(p) = prior {
            return steps - p;
        }
        previous.insert(o.clone(), steps);
    }
}

fn main() {
    let orbits: Vec<Orbit> = fs::read_to_string("input").expect("Couldn't read input").trim().split("\n").map(|orbit| Orbit::new(orbit)).collect();

    println!("{:#?}", orbits);

    let mut o = orbits.clone();
    for _ in 0..1000 {
        o = step(o);
    }
    
    println!("{:#?} Energy: {}", o, energy(&o));

    let x_orbit: Vec<Orbit> = orbits.iter().map(|o| Orbit {position: Vector(o.position.0, 0, 0), velocity: Vector(0, 0, 0) }).collect();
    
    let y_orbit: Vec<Orbit> = orbits.iter().map(|o| Orbit {position: Vector(0, o.position.1, 0), velocity: Vector(0, 0, 0) }).collect();
    
    let z_orbit: Vec<Orbit> = orbits.iter().map(|o| Orbit {position: Vector(0, 0, o.position.2), velocity: Vector(0, 0, 0) }).collect();

    println!("{} {} {}", cycle_length(x_orbit), cycle_length(y_orbit), cycle_length(z_orbit));

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cycle_length() {
        let orbits: Vec<Orbit> = r"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>".trim().split("\n").map(|orbit| Orbit::new(orbit)).collect();
        assert_eq!(cycle_length(orbits), 2772);
    }   
}

