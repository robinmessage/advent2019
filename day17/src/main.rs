use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashMap;
use std::convert::TryInto;

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
    //println!("instrValue: {}, offset: {}, access: {}", instrValue, offset, access);
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
    //println!("Set {} to {}", offset, to);
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
        //println!("R {} {}", opcode, access);
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

struct Screen {
    display: Vec<char>,
    width: i32,
    height: i32
}

fn get_screen(machine: &mut Machine) -> Screen {
    let mut input = vec![];

    let output = run(machine, &mut input);

    let mut display = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut first_row = true;
    let mut row = vec![];

    for c in &output {
        if *c == 10 { // New line
            let len = row.len() as i32;
            if first_row || len == width {
                height += 1;
                width = len;
                display.append(&mut row);
            } else {
                if row.len() > 0 {
                    println!("{}", row.iter().collect::<String>());
                }
            }
            first_row = false;
            row = vec![];
        } else {
            row.push(*c as u8 as char);
        }
    }

    return Screen { display, width, height };
}

fn get_pixel(screen: &Screen, x: i32, y: i32) -> char {
    if x < 0 || x >= screen.width || y < 0 || y >= screen.height {
        return ' ';
    }
    return screen.display[(x + y * screen.width) as usize];
}

fn paint_screen(screen: &Screen) {
    for y in 0..screen.height {
        println!("{}", (0..screen.width).map(|x| get_pixel(screen, x as i32, y as i32)).collect::<String>());
    }
}

fn find_intersection_scores(screen: &Screen) -> i32 {
    let mut score = 0;
    for y in 1..(screen.height - 1) {
        for x in 1..(screen.width -1) {
            if get_pixel(screen, x, y) == '#'
            && get_pixel(screen, x - 1, y) == '#'
            && get_pixel(screen, x + 1, y) == '#'
            && get_pixel(screen, x, y - 1) == '#'
            && get_pixel(screen, x, y + 1) == '#' {
                score += x * y;
            }
        }
    }
    return score;
}

fn find_robot(map: &Screen) -> (i32, i32) {
    for y in 0..(map.height) {
        for x in 0..(map.width) {
            match get_pixel(map, x, y) {
                '^' | 'v' | '<' | '>' => return (x, y),
                _ => continue
            }
        }
    }
    panic!("Robot is missing");
}

fn find_turn(map: &Screen, x: i32, y: i32, ox: i32, oy: i32) -> Option<char> {
    /*let mut ox: i32 = 0;
    let mut oy: i32 = 0;
    match get_pixel(map, x, y) {
        '^' => {ox = -1;},
        'v' => {ox = 1;},
        '<' => {oy = -1;},
        '>' => {oy = 1;},
        _ => {panic!("Not a robot direction")}
    }*/
    if get_pixel(map, x - ox, y + oy) == '#' {
        return Some('R');
    } else if get_pixel(map, x + ox, y - oy) == '#' {
        return Some('L');
    } else {
        return None;
    }
}

fn move_far(map: &Screen, mut x: i32, mut y: i32, dx: i32, dy: i32) -> i32 {
    let mut distance = 0;
    while get_pixel(map, x + dx, y + dy) == '#' {
        x += dx;
        y += dy;
        distance += 1;
    }
    return distance;
}


fn navigate(map: &Screen) -> String {
    let (mut x, mut y) = find_robot(map);

    let first_turn = 'R'; // Can't be bothered to code this
    let (mut dx, mut dy) = (1, 0); // Or this

    let mut instructions = vec![first_turn];
    
    instructions.push(',');

    for _ in 0..100 {
        // Move forward as far as possible
        let distance = move_far(map, x, y, dx, dy);

        if distance > 9 {
            instructions.push(('0' as u8 as i32 + (distance / 10)) as u8 as char);
        }
        instructions.push(('0' as u8 as i32 + (distance % 10)) as u8 as char);
        instructions.push(',');

        x += dx * distance;
        y += dy * distance;
        
        let turn = find_turn(map, x, y, dy, dx); // NB these are reversed as offsets for the next move, not current direction

        match turn {
            Some('L') => {
                let (nx, ny) = (dy, -dx);
                dx = nx;
                dy = ny;
            },
            Some('R') => {
                let (nx, ny) = (-dy, dx);
                dx = nx;
                dy = ny;
            },
            _ => {
                break;
            }
        }
        instructions.push(turn.unwrap());
        instructions.push(',');
    }
    instructions.pop(); // Ditch last comma

    return instructions.iter().collect();
}

fn find_seqs(route: &str) -> HashMap<String, i32> {
    let mut seqs = HashMap::new();
    let parts: Vec<&str> = route.split(",").collect();
    for i in 0..parts.len() {
        for j in (i + 1)..parts.len() {
            let seq = parts[i..j].join(",");
            *seqs.entry(seq).or_insert(0) += 1;
        }
    }
    seqs.retain(|k, v| k.len() < 20);
    return seqs;
}

fn _use_seqs(route: String, seqs: &Vec<String>, allocated: Vec<usize>) -> Option<(String, Vec<usize>)> {
    if allocated.len() == 3 {
        if route.chars().all(|b| b == ',' || b == 'A' || b == 'B' || b == 'C') {
            // Success
            return Some((route, allocated));
        } else {
            return None;
        }
    }
    let letter = (('A' as u8 + allocated.len() as u8) as char).to_string();
    for i in 0..seqs.len() {
        let seq = &seqs[i];
        let new_route = route.replace(seq, &letter);
        let mut new_allocated = allocated.clone();
        new_allocated.push(i);
        let result = _use_seqs(new_route, seqs, new_allocated);
        if result.is_some() {
            return result;
        }
    }
    return None;
}

fn use_seqs(route: &str, seqs: &HashMap<String, i32>) -> String {
    let mut seqs: Vec<String> = seqs.keys().map(|s| s.clone()).collect();
    seqs.sort_unstable_by_key(|seq| 0 - seq.len() as i32);

    let p = _use_seqs(route.to_string(), &seqs, vec![]);

    if let Some((program, seq_indexes)) = p {
        // TODO: Work out how to avoid so many string copies etc in Rust
        let internals = seq_indexes.iter().map(|i| seqs[*i].to_string()).collect::<Vec<String>>().join("\n");
        return program + "\n" + &internals + "\n" + "y" + "\n";
    } else {
        panic!("No program found");
    }
    // Find the next best sequence
    // Consume it, replace in route
    // If route incomplete
    //  if slots available, recurse, else fail
    // Else succeed
}

fn run_robot(machine: &mut Machine, map: &Screen) {

    let route = navigate(map);

    println!("{}", route);

    let seqs = find_seqs(&route);

    let prog = use_seqs(&route, &seqs);

    println!("{:#?}", prog);

    let mut input = prog.as_bytes().iter().map(|b| *b as Word).collect();

    let mut output = run(machine, &mut input);
    
    let score = output.pop().unwrap();

    println!("{}", output.iter().map(|b| *b as u8 as char).collect::<String>());

    println!("{}", score);
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        {
            let mut machine = Machine::new(parse(&line));

            let screen = get_screen(&mut machine);

            paint_screen(&screen);

            println!("{}", find_intersection_scores(&screen));
        }
        {
            let mut mem = parse(&line);
            mem[0] = 2;

            let mut machine = Machine::new(mem);

            let screen = get_screen(&mut machine);

            paint_screen(&screen);

            run_robot(&mut machine, &screen);
        }
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
    fn test_someone_else_route() {
        let route = "R,4,R,12,R,10,L,12,L,12,R,4,R,12,L,12,R,4,R,12,L,12,L,8,R,10,L,12,L,8,R,10,R,4,R,12,R,10,L,12,L,12,R,4,R,12,L,12,R,4,R,12,L,12,L,8,R,10,R,4,R,12,R,10,L,12";
        let code = use_seqs(&route, &find_seqs(&route));
        assert_eq!(code, "A,B,B,C,C,A,B,B,C,A\nR,4,R,12,R,10,L,12\nL,12,R,4,R,12\nL,12,L,8,R,10\ny\n");
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
