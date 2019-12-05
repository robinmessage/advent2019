use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn valid(number: i32) -> bool {
    let s = number.to_string();
    let b = s.as_bytes();
    // Two adjacent digits are the same
    let mut valid = false;
    for i in 0..(b.len() - 1) {
        if b[i] == b[i+1] {
            valid = true;
        }
    }
    if !valid {
        return false;
    }

    // Digits increase
    for i in 0..(b.len() - 1) {
        if b[i] > b[i+1] {
            return false;
        }
    }
    return true;
}

fn valid2(number: i32) -> bool {
    if (!valid(number)) {
        return false;
    }

    let s = " ".to_owned() + &number.to_string() + " ";
    let b = s.as_bytes();
    // Two adjacent digits are the same and not the same as neighbours
    let mut valid = false;
    for i in 0..(b.len() - 3) {
        if b[i] != b[i+1] && b[i+1] == b[i+2] && b[i+2] != b[i+3] {
            valid = true;
        }
    }

    return valid;
}

fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 353096..=843212 {
        count1 += if valid(i) { 1 } else { 0 };
        count2 += if valid2(i) { 1 } else { 0 };
    }
    println!("{} {}", count1, count2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_111111() {
        assert!(valid(111111));
    }

    #[test]
    fn test_223450() {
        assert!(!valid(223450));
    }

    #[test]
    fn test_123789() {
        assert!(!valid(123789));
    }

    #[test]
    fn test_112233() {
        assert!(valid2(112233));
    }

    #[test]
    fn test_123444() {
        assert!(!valid2(123444));
    }
    
    #[test]
    fn test_111122() {
        assert!(valid2(111122));
    }

}
