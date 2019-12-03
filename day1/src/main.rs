use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuelNeeded(weight: i32) -> i32 {
    let mut fuel: i32 = (weight / 3) - 2;
    if fuel > 0 {
        fuel += fuelNeeded(fuel);
    } else {
        return 0;
    }

    return fuel;
}

fn main() {
    let mut sum = 0;
    
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read");
        let weight: u32 = line.trim().parse()
            .expect("Please type a number!");

        sum += fuelNeeded(weight as i32);
    }

    println!("Fuel {}", sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_low() {
        assert_eq!(fuelNeeded(12), 2);
    }

    #[test]
    fn test_more() {
        assert_eq!(fuelNeeded(100756), 50346);
    }
}
