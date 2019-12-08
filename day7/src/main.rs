use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Machine {
    mem: Vec<i32>,
    ip: usize
}


impl Machine
{
    pub fn new(mem: Vec<i32>) -> Machine {
        Machine { mem: mem, ip: 0 }
    }
}

fn addr(machine: &Machine, offset: i32) -> usize {
    return (machine.ip as i32 + offset) as usize;
}

fn get(machine: &Machine, offset: i32, access: i32) -> i32 {
    let mut a = access;
    for _ in 1..offset {
        a /= 10;
    }
    a = a % 10;
    let addr = addr(machine, offset);
    return match a {
        0 => machine.mem[machine.mem[addr] as usize],
        1 => machine.mem[addr],
        _ => panic!("Unknown access type {} (offset: {} access: {})", a, offset, access)
    };
}

fn run(machine: &mut Machine, input: &mut Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    loop {
        let opcode = machine.mem[machine.ip] % 100;
        let access = machine.mem[machine.ip] / 100;
        match opcode {
            1 => {
                let target = machine.mem[addr(machine, 3)] as usize;
                machine.mem[target] = get(machine, 1, access) + get(machine, 2, access);
                machine.ip += 4;
            },
            2 => {
                let target = machine.mem[addr(machine, 3)] as usize;
                machine.mem[target] = get(machine, 1, access) * get(machine, 2, access);
                machine.ip += 4;
            },
            3 => {
                if input.len() == 0 {
                    return output;
                }
                let target = machine.mem[addr(machine, 1)] as usize;
                machine.mem[target] = input.remove(0);
                machine.ip += 2;
            },
            4 => {
                output.push(get(machine, 1, access));
                machine.ip += 2;
            },
            5 => {
                let val = get(machine, 1, access);
                if val != 0 {
                    machine.ip = get(machine, 2, access) as usize;
                } else {
                    machine.ip += 3;
                }
            },
            6 => {
                let val = get(machine, 1, access);
                if val == 0 {
                    machine.ip = get(machine, 2, access) as usize;
                } else {
                    machine.ip += 3;
                }
            },
            7 => {
                let target = machine.mem[addr(machine, 3)] as usize;
                machine.mem[target] = if get(machine, 1, access) < get(machine, 2, access) {1} else {0};
                machine.ip += 4;
            },
            8 => {
                let target = machine.mem[addr(machine, 3)] as usize;
                machine.mem[target] = if get(machine, 1, access) == get(machine, 2, access) {1} else {0};
                machine.ip += 4;
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

fn permutations(of: Vec<i32>) -> Vec<Vec<i32>> {
    if of.len() == 1 {
        return vec![of];
    } else {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..of.len() {
            let mut without = of.clone();
            without.remove(i);
            let ps = permutations(without);
            for mut p in ps {
                p.push(of[i]);
                result.push(p);
            }
        }
        return result;
    }       
}

fn runPhases(line: &str, p: &Vec<i32>) -> i32 {
    let mut amps: Vec<Machine> = Vec::new();

    let mut o = 0;
    let mut done = false;
    let mut lastO = 0;
    while !done {
        let mut amp = 0;
        for x in p {
            let mut input = Vec::new();

            if amp >= amps.len() {
                amps.push(Machine::new(parse(&line)));
                input.push(*x);
            }
            
            let ampMem = &mut amps[amp];

            input.push(o);
            let mut output = run(ampMem, &mut input);
            if output.len() == 0 {
                done = true;
            } else {
                while output.len() > 0 {
                    o = output.remove(0);
                    input.push(o);
                }
                if amp == 4 {
                    lastO = o;
                }
            }
            //println!("- {} {} {} {}", amp, o, done, input.len()); 
            amp+=1;
        }
    }
    //println!("END {} {} {:?}", lastO, o, p);
    return lastO;
}


fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");


        /*for p in permutations(vec![0,1,2,3,4]) {
            let mut o = 0;
            for x in &p {
                let mut mem = parse(&line);

                let mut input = Vec::new();
                input.push(*x);
                input.push(o);
                let output = run(&mut mem, &mut input);
                o = output[0];
            }
            println!("{} {:?}", o, p);
        }*/

        for p in permutations(vec![5,6,7,8,9]) {
            let output = runPhases(&line, &p);
            println!("{} {:?}", output, p);
        }
    }
}

/*
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

    #[test]
    fn test_phase_programs() {
        assert_eq!(runPhases("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", vec![9, 8, 7, 6, 5]), 139629729);
    }
}*/
