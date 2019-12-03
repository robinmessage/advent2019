use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn run(mem: &mut Vec<usize>) {
    let mut ip: usize = 0;
    loop {
        match mem[ip] {
            1 => {
                let target = mem[ip + 3];
                mem[target] = mem[mem[ip + 1]] + mem[mem[ip + 2]];
            },
            2 => {
                let target = mem[ip + 3];
                mem[target] = mem[mem[ip + 1]] * mem[mem[ip + 2]];
            },
            99 => {
                return;
            },
            _ => panic!("Incorrect opcode")
        }
        ip += 4;
    }
}

fn parse(line: &str) -> Vec<usize> {
    return line.split(",").map(|item| item.trim().parse()
        .expect("Please type a number!")).collect::<Vec<usize>>();
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        let mut mem = parse(&line);

        for noun in 0..=99 {
            for verb in 0..=99 {
                mem = parse(&line);
                mem[1] = noun;
                mem[2] = verb;
                run(&mut mem);
                if mem[0] == 19690720 {
                    println!("{}", mem[0]);
                    println!("Result {}", noun * 100 + verb);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_case(start: &str, expected: &str) {
        let expected = parse(expected);
        let mut actual = parse(start);
        run(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_one() {
        test_case("1,0,0,0,99", "2,0,0,0,99");
        test_case("2,3,0,3,99 ", " 2,3,0,6,99");
        test_case("2,4,4,5,99,0 ", " 2,4,4,5,99,9801");
        test_case("1,1,1,4,99,5,6,0,99 ", " 30,1,1,4,2,5,6,0,99");
    }
}
