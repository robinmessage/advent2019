use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;
use regex::Regex;

fn phase_array(phase: i32, length: usize) -> Vec<i32> {
    let signals = vec![0, 1, 0, -1];
    let mut output = vec![];

    let mut i = 1;
    let mut j = 0;
    while output.len() < length {
        if i == phase {
            i = 0;
            j += 1;
        }
        if j == signals.len() {
            j = 0;
        }
        output.push(signals[j]);
        i += 1;
    }

    return output;
}

fn parse_signal(signal: &str) -> Vec<i32> {
    return signal.as_bytes().iter().map(|b| (b - ('0' as u8)) as i32).collect();
}

fn calculate_phase(signal: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![];
    for i in 0..signal.len() {
        let mut sum: i32 = 0;
        let coeffs = phase_array(1 + i as i32, signal.len());
        for j in 0..signal.len() {
            sum += signal[j] * coeffs[j];
        }
        output.push(sum.abs() % 10); // Odd but correct for this challenge
    }
    return output;
}

fn calculate_phases(signal: &Vec<i32>, phase_count: usize) -> Vec<i32> {
    let mut output = calculate_phase(signal);
    for _ in 1..phase_count {
        output = calculate_phase(&output);
    }
    return output;
}

fn calculate_fast_phase(mut signal: Vec<i32>) -> Vec<i32> {
    for i in (0..(signal.len() - 1)).rev() {
        signal[i] += signal[i + 1];
    }

    for s in signal.iter_mut() {
        *s = s.abs() % 10;
    }

    return signal;
}

fn calculate_fast_phases(signal: &Vec<i32>, phase_count: usize) -> Vec<i32> {
    let mut output: Vec<i32> = signal.iter().cloned().collect();
    for _ in 0..phase_count {
        output = calculate_fast_phase(output);
    }
    return output;
}

fn main() {
    let input: Vec<i32> = parse_signal(fs::read_to_string("input").expect("Couldn't read input").trim());
    //let output = calculate_phases(&input, 100);
    //println!("{}", output.iter().take(8).map(|d| d.to_string()).collect::<String>());

    let skip = 5976277;
    let total = input.len() * 10000;
    let take = total - skip;

    println!("{} {} {}", total, skip, take);

    let relevant_input: Vec<i32> = input.iter().cloned().cycle().skip(skip).take(take).collect();

    let output = calculate_fast_phases(&relevant_input, 100);
    println!("{}", output.iter().take(8).map(|d| d.to_string()).collect::<String>());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_phase_array() {
        assert_eq!(phase_array(1, 4), vec![1, 0, -1, 0]);
        assert_eq!(phase_array(1, 8), vec![1, 0, -1, 0, 1, 0, -1, 0,]);
        assert_eq!(phase_array(2, 15), vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(phase_array(3, 9), vec![0, 0, 1, 1, 1, 0, 0, 0, -1]);
    }
    
    #[test]
    fn test_parse_signal() {
        assert_eq!(parse_signal("1234"), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_calculate_phase() {
        assert_eq!(calculate_phase(&parse_signal("12345678")), parse_signal("48226158"));
    }
    
    #[test]
    fn test_calculate_phases() {
        assert_eq!(calculate_phases(&parse_signal("12345678"), 4), parse_signal("01029498"));
    }
}

