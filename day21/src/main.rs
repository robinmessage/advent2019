use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashMap;
use std::collections::HashSet;

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

fn to_ascii(output: &Vec<Word>) -> String {
    output.iter().map(|c| *c as u8 as char).collect()
}

fn from_ascii(input: &str) -> Vec<Word> {
    input.as_bytes().iter().map(|c| *c as u8 as Word).collect()
}

fn states(count: usize, sight: usize) -> Vec<Vec<bool>> {
    let mut states = vec![];
    for i in 0..(1 << count) {
        let mut state = vec![];
        for j in 0..count {
            state.push(i & (1 << j) != 0);
        }
        for _ in 0..sight {
            state.push(true);
        }
        states.push(state);
    }
    return states;
}

fn passable_states(mut states: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    states.retain(|state| {
        let mut gap = 0;
        for j in 0..(state.len()) {
            if state[j] {
                gap = 0;
            } else {
                gap += 1;
                if gap == 4 {
                    return false;
                }
            }
        }
        return true;
    });
    return states;
}

fn solve(state: &Vec<bool>, pos: usize, sight: usize) -> Vec<Vec<usize>> {
    if pos + sight >= state.len() {
        return vec![vec![]];
    }
    let mut results = vec![];
    // Jump
    if state[pos + 3] {
        // Jump at pos
        let mut result = solve(state, pos + 4, sight);
        for r in result.iter_mut() {
            r.insert(0, pos);
        }
        results.append(&mut result);
    }
    // Don't jump
    if state[pos] {
        // No jump here
        results.append(&mut solve(state, pos + 1, sight));
    }
    return results;
}

fn print_states(states: &Vec<Vec<bool>>) {
    for state in states {
        println!("{}", state.iter().map(|s| if *s {'#'} else {' '}).collect::<String>());
    }
}

fn drop_impossible(mut solutions: Vec<(&Vec<bool>,  Vec<Vec<usize>>)>) -> Vec<(&Vec<bool>,  Vec<Vec<usize>>)> {
    solutions.retain(|(_state, solution)| solution.len() > 0);
    return solutions;
}

fn solve_all(states: &Vec<Vec<bool>>, sight: usize) -> Vec<(&Vec<bool>,  Vec<Vec<usize>>)> {
    drop_impossible(states.iter().map(|state| (state, solve(&state, 0, sight))).collect())
}

fn print_states_and_solutions(states: &Vec<Vec<bool>>, sight: usize) {
    for (state, solution) in solve_all(states, sight) {
        println!("{} {:?}", state.iter().map(|s| if *s {'#'} else {' '}).collect::<String>(), solution);
    }
}

fn expand_solution(solution: &Vec<Vec<usize>>, len: usize) -> Vec<Vec<Option<bool>>> {
    let mut results = vec![];
    for solution in solution {
        let mut i = 0;
        let mut j = 0;
        let mut result = vec![];
        while i < len {
            if j < solution.len() && solution[j] == i {
                // Jump here
                result.push(Some(true));
                result.push(None);
                result.push(None);
                result.push(None);
                i += 4;
                j += 1;
            } else {
                result.push(Some(false));
                i += 1;
            }
        }
        results.push(result);
    }
    return results;
}

type Decisions = HashMap<Vec<bool>, (i32, i32)>;

fn to_decisions(solutions: &Vec<(&Vec<bool>,  Vec<Vec<usize>>)>, sight: usize) -> Decisions {
    let mut choice: Decisions = HashMap::new();
    for (state, solution) in solutions {
        // Track valid solutions
        let expanded_solutions = expand_solution(&solution, state.len());
        for pos in 0..=(state.len() - sight) {
            let visible_state = &state[pos..(pos+sight)];
            let mut jump_found = false;
            let mut walk_found = false;
            for expanded_solution in &expanded_solutions {
                if let Some(x) = expanded_solution[pos] {
                    if x {
                        jump_found = true;
                    } else {
                        walk_found = true;
                        if !visible_state[0] {
                            // Shouldn't happen
                            panic!("Error pos:{} state:{}, expanded_solution:{:?}, solution:{:?}", pos, state_to_string(state), expanded_solution, solution);
                        }
                    }
                }
            }
            if walk_found && !jump_found {
                choice.entry(visible_state.to_vec()).or_insert((0, 0)).0 += 1;
            } else if jump_found && !walk_found {
                choice.entry(visible_state.to_vec()).or_insert((0, 0)).1 += 1;
            }
        }
    }
    return choice;
}

fn state_to_string(state: &Vec<bool>) -> String {
    state.iter().map(|s| if *s {'#'} else {' '}).collect::<String>()
}

fn print_decisions(decisions: &Decisions) {
    for (visible_state, (walk, jump)) in decisions {
        println!("{} Jump: {} Walk: {}", state_to_string(visible_state), jump, walk);
    }
}

fn decision_set(decisions: &Decisions) -> HashSet<&Vec<bool>> {
    let mut jumps = HashSet::new();
    for (visible_state, (walk, jump)) in decisions {
        if jump > walk {
            jumps.insert(visible_state);
        }
    }
    return jumps;
}

fn nth_is<'a>(what: bool, n: usize, decisions: &'a HashSet<&'a Vec<bool>>) -> HashSet<&'a Vec<bool>> {
    let mut result = HashSet::new();
    for decision in decisions {
        if decision[n] == what {
            result.insert(*decision);
        }
    }
    return result;
}

fn do_insert(of: Option<bool>, got: &Vec<Option<bool>>, into: &mut HashSet<Vec<Option<bool>>>) {
    let mut to_insert = got.clone();
    to_insert.insert(0, of);
    into.insert(to_insert);
}


fn simplify(decisions: &HashSet<&Vec<bool>>, n: usize, count: usize) -> HashSet<Vec<Option<bool>>> {
    let mut results = HashSet::new();
    if n == count {
        if decisions.len() > 0 {
            results.insert(vec![]);
        }
        return results;
    }
    
    // Split
    let trues = simplify(&nth_is(true, n, decisions), n + 1, count);
    let mut falses = simplify(&nth_is(false, n, decisions), n + 1, count);

    // Combine
    for true_item in trues.iter() {
        if falses.contains(true_item) {
            do_insert(None, true_item, &mut results);
            falses.remove(true_item);
        } else {
            do_insert(Some(true), true_item, &mut results);
        }
    }
    for false_item in falses.iter() {
        do_insert(Some(false), false_item, &mut results);
    }

    return results;
}

fn print_simplified_decisions(ds: &HashSet<Vec<Option<bool>>>) {
    fn what(d: &Vec<Option<bool>>) -> String {
        d.iter().enumerate().map(|(i, e)| {
            if *e == Some(true) {('A' as u8 + i as u8) as char}
            else if *e == Some(false) {('a' as u8 + i as u8) as char}
            else {'_'}
        }).collect()
    }
    for d in ds {
        println!("{}", what(d));
    }
}

fn main() {

    /*let states8 = passable_states(states(8));

    let decisions = to_decisions(&solve_all(&states8), 4);
    println!("{:?}", print_decisions(&decisions));

    let ds = decision_set(&decisions);

    print_simplified_decisions(&simplify(&ds, 0, 4));*/

    let states12 = passable_states(states(9, 9));

    let decisions = to_decisions(&solve_all(&states12, 9), 9);
    println!("{:?}", print_decisions(&decisions));

    let ds = decision_set(&decisions);

    print_simplified_decisions(&simplify(&ds, 0, 9));

    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        
        {let mut machine = Machine::new(parse(&line));

        println!("{}", to_ascii(&run(&mut machine, &mut vec![])));

// !##.#.
// !.....
//  ABCD
/*        let mut prog = from_ascii(r"NOT A J
NOT C T
AND D T
OR T J
WALK
");*/

/*
aD
ABcD
AbD
*/
        let mut prog = from_ascii(r"NOT A J
AND D J
NOT C T
AND B T
AND D T
OR T J
NOT B T
AND D T
OR T J
WALK
");

        let mut output = run(&mut machine, &mut prog);

        println!("{}", to_ascii(&output));
        println!("Result: {}", output.pop().unwrap());
        }
        {let mut machine = Machine::new(parse(&line));

        println!("{}", to_ascii(&run(&mut machine, &mut vec![])));

// !##.#.    // covered
// !##.v#.#v // covered
// !#.##...#
// !##.v..##
// !###o.#.x#
// !####.#..#
// !####.v..#
// !###x#..##
// !####.v.#.v..####
// !##.x.v..####
//  ABCDEFGHI
        /*let mut prog = from_ascii(r"NOT A J
NOT E T
AND D T
AND H T
AND I T
OR T J
NOT E T
AND G T
OR C T
NOT T T
AND D T
OR T J
RUN
");*/

/*
a:
a__DEF___
a__DEf_H_
a__DEf_hI
a__De__H_

#Ab_DEFgH_
#Ab_DEf_H_
#Ab_De__H_
#ABcD_FgH_
#ABcD_f_H_

H && D:
_b__e____
_b__Ef___
_b__EFg__

__Bc__Fg__
__Bc__f___

*/
// x = (!F | (F & !G))
// x = !F | !G
// !x = F & G
// (B & !C & x) | (!B & (!E | (E & x)))
// (B & !C & x) | (!B & (!E | x))
// (B & !C & x) = (B & !C & (!F | !G)) = !(!B | C | !x) = !(!B | C | (F & G))
// (!B & (!E | x)) = (!B & (!E | (!F | !G))) = !(B | (E & !x)) = !(B | (E & F & G))
/*
// Sunday morning hopefully got it
// (B & !C & (!F | !G))
NOT G T
NOT F J
OR J T
NOT C J
AND B J
AND T J
// !(B | (E & !x)) 
NOT T T
AND E T
OR B T
NOT T T
OR T J

// D & H & (complex)
AND D J
AND H J
// | !A
NOT A T
OR T J
*/
/*
NOT G T
NOT F J
OR J T
NOT E J
OR T J
NOT B T
AND T J
NOT G T
*/
        let mut prog = from_ascii(
r"NOT G T
NOT F J
OR J T
NOT C J
AND B J
AND T J
NOT T T
AND E T
OR B T
NOT T T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN
");

/*
Sunday afternoon, seeing that you can simplify the following pairs
...Xy...
...x_...
into ...(x or y)... (where the ...s before are the same, and the afters also)

So, we've got NOT A as a simple condition for those cases
The first part of all the other cases are then covered by NOT B OR NOT C AND D.
The second part of the other cases are:
Ef_Hi
Ef_Hi
ef_H_
e__H_

These don't simplify, and you can jump on case A__DEf__I, but you don't need to (you can walk one then jump to E)
I wonder if I make my search prefer jumping, will it find a simpler expression?
*/
        

        let mut output = run(&mut machine, &mut prog);

        println!("{}", to_ascii(&output));
        println!("Result: {}", output.pop().unwrap());
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

    fn to_bools(x: &str) -> Vec<bool> {
        x.as_bytes().iter().map(|c| *c == '#' as u8).collect()
    }
    
    #[test]
    fn test_simplify1() {
        let mut a1 = HashSet::new();
        let a11 = vec![false];
        a1.insert(&a11);
        let mut e1 = HashSet::new();
        e1.insert(vec![Some(false)]);
        assert_eq!(simplify(&a1, 0, 1), e1);
    }
    
    #[test]
    fn test_simplify2() {
        let mut a1 = HashSet::new();
        let a11 = vec![false, true];
        a1.insert(&a11);
        let a12 = vec![false, false];
        a1.insert(&a12);
        let mut e1 = HashSet::new();
        e1.insert(vec![Some(false), None]);
        assert_eq!(simplify(&a1, 0, 2), e1);
    }

    #[test]
    fn test_solve() {
        //                         0123456789012 
        assert_eq!(solve(&to_bools("##   #  ####"), 0), vec![[2, 6]]);
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
