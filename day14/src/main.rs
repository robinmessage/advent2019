use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
struct Product(String, i32);

type ProductSet = Vec<Product>;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
struct ReactionData(i32, ProductSet);

type Reactions = HashMap<String, ReactionData>;

fn parse_reactions(input: &str) -> Reactions {
    lazy_static! {
        static ref PARSE_PRODUCT: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    }

    let mut reactions = Reactions::new();

    for reaction in input.split("\n") {

        let mut products = ProductSet::new();

        for cap in PARSE_PRODUCT.captures_iter(reaction) {
            products.push(Product(cap[2].to_string(), cap[1].parse().unwrap())); 
        }

        let finalProduct = products.pop().unwrap();

        reactions.insert(finalProduct.0, ReactionData(finalProduct.1, products));
    }

    return reactions;
}

fn reverse_reactions(reactions: &Reactions, order: &Vec<String>) -> i32 {
    // Get to ore
    let mut reactants = HashMap::<String, i32>::new();

    reactants.insert("FUEL".to_string(), 1);

    for name in order.iter().rev() {
        let amount = reactants.get(name).unwrap();
        println!("Need {} of {}", amount, name);

        if name == "ORE" {
            return *amount;
        }

        let reaction = reactions.get(name).unwrap();
        let multiple = (amount + reaction.0 - 1) / reaction.0; // Round up
        for input in reaction.1.iter() {
            *reactants.entry(input.0.to_string()).or_insert(0) += input.1 * multiple;
        }
    }

    panic!("Never got to ORE");
}

fn topo_sort(reactions: &Reactions) -> Vec<String> {
    struct TopoEnv<'a> {
        reactions: &'a Reactions,
        order: Vec<String>,
        visited: HashSet<String>
    }

    fn depth_first(env: &mut TopoEnv, key: &str) {
        if env.visited.contains(key) {
            return;
        }
        let reaction = env.reactions.get(key);
        if let Some(reaction) = reaction {
            for input in reaction.1.iter() {
                depth_first(env, &input.0);
            }
        }
        env.order.push(key.to_string());
        env.visited.insert(key.to_string());
    };

    // Depth-first search from ORE
    let mut env = TopoEnv {
        reactions,
        order: vec![],
        visited: HashSet::new()
    };
    depth_first(&mut env, "FUEL");

    return env.order;
}

fn create_maximum_from(ore: i64, reactions: &Reactions, order: &Vec<String>) -> i64 {
    // Get to ore
    let mut reactants_needed = HashMap::<String, i64>::new();
    
    let mut ore = ore;
    let mut fuel = 0;

    while ore > 0 {
        reactants_needed.insert("FUEL".to_string(), 1);

        for name in order.iter().rev() {
            let amount: &mut i64 = reactants_needed.entry(name.to_string()).or_insert(0);
            
            if name == "ORE" {
                ore -= *amount;
                *amount = 0;
                if ore >= 0 {
                    fuel += 1;
                }
                continue;
            }

            if *amount > 0 {
                let reaction = reactions.get(name).unwrap();
                let multiple = (*amount + reaction.0 as i64 - 1) / reaction.0 as i64; // Round up
                *amount -= reaction.0 as i64 * multiple;
                for input in reaction.1.iter() {
                    *reactants_needed.entry(input.0.to_string()).or_insert(0) += input.1 as i64 * multiple;
                }
            }
        }
    }

    return fuel;
}

fn main() {
    let reactions: Reactions = parse_reactions(fs::read_to_string("input").expect("Couldn't read input").trim());

    println!("{:#?}", reactions);

    let order = topo_sort(&reactions);
    println!("{}", reverse_reactions(&reactions, &order));

    println!("{}", create_maximum_from(1000000000000, &reactions, &order));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple() {
        let r = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let reactions = parse_reactions(r);
        let order = topo_sort(&reactions);
        assert_eq!(reverse_reactions(&reactions, &order), 31);
    }

    #[test]
    fn test_ore_for_fuel() {
        let r =r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let reactions = parse_reactions(r);
        let order = topo_sort(&reactions);
        assert_eq!(create_maximum_from(1000000000000, &reactions, &order), 5586022);
    }
}

