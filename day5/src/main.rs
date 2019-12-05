use std::fs::File;
use std::io::{prelude::*, BufReader};

fn addr(ip: usize, offset: i32) -> usize {
    return (ip as i32 + offset) as usize;
}

fn get(mem: &mut Vec<i32>, ip: usize, offset: i32, access: i32) -> i32 {
    let mut a = access;
    for _ in 1..offset {
        a /= 10;
    }
    a = a % 10;
    let addr = addr(ip, offset);
    println!("get a: {} offset:{} addr: {}", a, offset, addr);
    return match a {
        0 => mem[mem[addr] as usize],
        1 => mem[addr],
        _ => panic!("Unknown access type {} (offset: {} access: {})", a, offset, access)
    };
}

fn run(mem: &mut Vec<i32>, input: &mut Vec<i32>) -> Vec<i32> {
    let mut ip: usize = 0;
    let mut output = Vec::new();
    loop {
        let opcode = mem[ip] % 100;
        let access = mem[ip] / 100;
        println!("opcode: {} access: {} ip: {}", opcode, access, ip);
        match opcode {
            1 => {
                let target = mem[addr(ip, 3)] as usize;
                mem[target] = get(mem, ip, 1, access) + get(mem, ip, 2, access);
                ip += 4;
            },
            2 => {
                let target = mem[addr(ip, 3)] as usize;
                println!("target: {}", target);
                mem[target] = get(mem, ip, 1, access) * get(mem, ip, 2, access);
                ip += 4;
            },
            3 => {
                let target = mem[addr(ip, 1)] as usize;
                mem[target] = input.remove(0);
                ip += 2;
            },
            4 => {
                output.push(get(mem, ip, 1, access));
                ip += 2;
            },
            5 => {
                let val = get(mem, ip, 1, access);
                if val != 0 {
                    ip = get(mem, ip, 2, access) as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                let val = get(mem, ip, 1, access);
                if val == 0 {
                    ip = get(mem, ip, 2, access) as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                let target = mem[addr(ip, 3)] as usize;
                mem[target] = if get(mem, ip, 1, access) < get(mem, ip, 2, access) {1} else {0};
                ip += 4;
            },
            8 => {
                let target = mem[addr(ip, 3)] as usize;
                mem[target] = if get(mem, ip, 1, access) == get(mem, ip, 2, access) {1} else {0};
                ip += 4;
            },
            99 => {
                return output;
            },
            _ => panic!("Incorrect opcode")
        }
    }
}

fn parse(line: &str) -> Vec<i32> {
    return line.split(",").map(|item| item.trim().parse()
        .expect("Please type a number!")).collect::<Vec<i32>>();
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        let mut mem = parse(&line);

        let mut input = Vec::new();
        input.push(5);
        let output = run(&mut mem, &mut input);
        println!("{}", output.into_iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_io(start: &str, expected: &str, input: &str) {
        let expected = parse(expected);
        let mut actual = parse(start);
        let mut input = parse(input);
        let output = run(&mut actual, &mut input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_input_to_output_copy() {
        test_io("3,0,4,0,99", "17", "17");
    }
    
    fn test_mem(start: &str, expected: &str) {
        let expected = parse(expected);
        let mut actual = parse(start);
        run(&mut actual, &mut vec![]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_access() {
        test_mem("1102,3,33,4,17", "1102,3,33,4,99");
        test_mem("2,3,0,3,99 ", " 2,3,0,6,99");
        test_mem("2,4,4,5,99,0 ", " 2,4,4,5,99,9801");
        test_mem("1,1,1,4,99,5,6,0,99 ", " 30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn test_comparison_functions() {
        let prog_eq8 = "3,9,8,9,10,9,4,9,99,-1,8";
        test_io(prog_eq8, "0", "4");
        test_io(prog_eq8, "1", "8");
        test_io(prog_eq8, "0", "12");

        let prog_lt8 = "3,9,7,9,10,9,4,9,99,-1,8";
        test_io(prog_lt8, "1", "4");
        test_io(prog_lt8, "0", "8");
        test_io(prog_lt8, "0", "12");

        let prog_eq8i = "3,3,1108,-1,8,3,4,3,99";
        test_io(prog_eq8i, "0", "4");
        test_io(prog_eq8i, "1", "8");
        test_io(prog_eq8i, "0", "12");

        println!("A");
        
        let prog_lt8i = "3,3,1107,-1,8,3,4,3,99";
        test_io(prog_lt8i, "1", "4");
        test_io(prog_lt8i, "0", "8");
        test_io(prog_lt8i, "0", "12");

        let prog_long = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        test_io(prog_long, "999", "4");
        test_io(prog_long, "1000", "8");
        test_io(prog_long, "1001", "12");
    }
}
