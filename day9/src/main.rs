use std::fs::File;
use std::io::{prelude::*, BufReader};

type Word = i64;

struct Machine {
    mem: Vec<Word>,
    ip: usize,
    relative_base: Word
}


impl Machine
{
    pub fn new(mem: Vec<Word>) -> Machine {
        Machine { mem: mem, ip: 0, relative_base: 0 }
    }
}

type AddrOrImm = Result<usize, Word>;

fn getAt(machine: &Machine, addr: usize) -> Word {
    if addr >= machine.mem.len() {
        return 0;
    }
    return machine.mem[addr];
}

fn addr(machine: &Machine, offset: Word, access: Word) -> AddrOrImm {
    let mut a = access;
    for _ in 1..offset {
        a /= 10;
    }
    a = a % 10;
    let instrValue = getAt(machine, machine.ip + offset as usize);
    println!("instrValue: {}, offset: {}, access: {}", instrValue, offset, access);
    return match a {
        0 => Ok(instrValue as usize),
        1 => Err(instrValue),
        2 => Ok((machine.relative_base + instrValue) as usize),
        _ => panic!("Unknown access type {} (offset: {} access: {})", a, offset, access)
    };
}

fn get(machine: &Machine, offset: Word, access: Word) -> Word {
    let a = addr(machine, offset, access);
    return match a {
        Ok(address) => getAt(machine, address),
        Err(immediate) => immediate
    };
}

fn set(machine: &mut Machine, offset: Word, to: Word) {
    println!("Set {} to {}", offset, to);
    let offset_as_usize = offset as usize;
    if machine.mem.len() <= offset_as_usize {
        machine.mem.resize(offset_as_usize + 1, 0);
    }
    machine.mem[offset_as_usize] = to;
}

fn run(machine: &mut Machine, input: &mut Vec<Word>) -> Vec<Word> {
    let mut output = Vec::new();
    loop {
        let opcode = machine.mem[machine.ip] % 100;
        let access = machine.mem[machine.ip] / 100;
        println!("R {} {}", opcode, access);
        match opcode {
            1 => {
                let target = addr(machine, 3, access).expect("Can't write to an immediate") as Word;
                set(machine, target, get(machine, 1, access) + get(machine, 2, access));
                machine.ip += 4;
            },
            2 => {
                let target = addr(machine, 3, access).expect("Can't write to an immediate") as Word;
                set(machine, target, get(machine, 1, access) * get(machine, 2, access));
                machine.ip += 4;
            },
            3 => {
                if input.len() == 0 {
                    return output;
                }
                let target = addr(machine, 1, access).expect("Can't write to an immediate") as Word;
                set(machine, target, input.remove(0));
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
                let target = addr(machine, 3, access).expect("Can't write to an immediate") as Word;
                set(machine, target, if get(machine, 1, access) < get(machine, 2, access) {1} else {0});
                machine.ip += 4;
            },
            8 => {
                let target = addr(machine, 3, access).expect("Can't write to an immediate") as Word;
                set(machine, target, if get(machine, 1, access) == get(machine, 2, access) {1} else {0});
                machine.ip += 4;
            },
            9 => {
                machine.relative_base += get(machine, 1, access);
                machine.ip += 2;
            },
            99 => {
                return output;
            },
            _ => panic!("Incorrect opcode")
        }
    }
}

fn parse(line: &str) -> Vec<Word> {
    return line.split(",").map(|item| item.trim().parse()
        .expect("Please type a number!")).collect::<Vec<Word>>();
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");

        let mut machine = Machine::new(parse(&line));
        
        let mut input = Vec::new();
        input.push(2);

        let output = run(&mut machine, &mut input);

        println!("{:?}", output);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_io(start: &str, expected: &str, input: &str) {
        let expected = parse(expected);
        let mut actual = Machine::new(parse(start));
        let mut input = parse(input);
        let output = run(&mut actual, &mut input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_quine_to_output() {
        let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        test_io(quine, quine, "0"); // Ignores input
    }

    #[test]
    fn test_input_to_output_copy() {
        test_io("3,0,4,0,99", "17", "17");
    }
    
    fn test_mem(start: &str, expected: &str) {
        let expected = parse(expected);
        let mut actual = Machine::new(parse(start));
        run(&mut actual, &mut vec![]);
        assert_eq!(actual.mem, expected);
    }

    #[test]
    fn test_access() {
        test_mem("1,1,1,4,99,5,6,0,99 ", " 30,1,1,4,2,5,6,0,99");
        test_mem("1102,3,33,4,17", "1102,3,33,4,99");
        test_mem("2,3,0,3,99 ", " 2,3,0,6,99");
        test_mem("2,4,4,5,99,0 ", " 2,4,4,5,99,9801");
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
