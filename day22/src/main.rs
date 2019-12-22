use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use modinverse::modinverse;

type Deck = Vec<i64>;

fn deal(size: usize) -> Deck {
    let mut deck = Vec::new();
    for i in 0..size {
        deck.push(i as i64);
    }
    deck
}

fn restack(mut deck: Deck) -> Deck {
    deck.reverse();
    deck
}

fn cut(mut deck: Deck, n: i64) -> Deck {
    if n > 0 {
        deck.rotate_left(n as usize)
    } else {
        deck.rotate_right((-n) as usize)
    }
    deck
}

fn redeal(deck: Deck, n: usize) -> Deck {
    // Can't work out how to do this in-place
    let mut result = Vec::new();
    result.resize(deck.len(), 0);
    let len = deck.len();
    let new_pos = |i: usize| -> usize {
        (i * n) % len
    };
    for i in 0..len {
        result[new_pos(i)] = deck[i];
    }
    result
}

#[derive(Debug)]
enum Action {
    Restack,
    Cut(i64),
    Redeal(usize)
}

fn parse(line: &str) -> Action {
    if line == "deal into new stack" {
        Action::Restack
    } else if line.starts_with("cut ") {
        Action::Cut(line[4..line.len()].parse().expect("Expected an integer"))
    } else if line.starts_with("deal with increment ") {
        Action::Redeal(line[20..line.len()].parse().expect("Expected a usize"))
    } else {
        panic!("Don't know how to {}", line);
    }
}

fn act(deck: Deck, action: &Action) -> Deck {
    match action {
        Action::Restack => restack(deck),
        Action::Cut(n) => cut(deck, *n),
        Action::Redeal(n) => redeal(deck, *n)
    }
}

fn execute_all<'a, I>(mut deck: Deck, actions: I) -> Deck
    where I: IntoIterator<Item = &'a Action> {
    for action in actions.into_iter() {
        deck = act(deck, action);
    }
    deck
}

#[derive(Debug, Copy, Clone)]
struct CombinedAction {
    size: i64,
    restacked: bool,
    cut: i64,
    redeal: usize
}

impl CombinedAction {
    fn empty(size: usize) -> CombinedAction {
        CombinedAction { size: size as i64, restacked: false, cut: 0, redeal: 1 }
    }

    fn execute(&self, mut deck: Deck) -> Deck {
        deck = redeal(deck, self.redeal);
        deck = cut(deck, self.cut);
        if self.restacked {
            deck = restack(deck);
        }
        deck
    }

    fn combine(&self, action: &Action) -> CombinedAction {
        match action {
            Action::Restack => CombinedAction { restacked: !self.restacked, ..*self },
            Action::Cut(n) => {
                CombinedAction {
                    cut: (if self.restacked {
                            self.cut - *n
                        } else {
                            self.cut + *n
                        }) % self.size,
                    ..*self
                }
            },
            Action::Redeal(n) => {
                let redeal = (self.redeal as i128 * *n as i128 % self.size as i128) as usize;
                let cut = (self.cut as i128 * *n as i128 % self.size as i128) as i64;
                if self.restacked {
                    CombinedAction { redeal, cut: cut + 1 - *n as i64, ..*self }
                } else {
                    CombinedAction { redeal, cut, ..*self }
                }
            }
        }
    }

    fn merge(&self, other: &CombinedAction) -> CombinedAction {
        let mut result = self.clone();
        result = result.combine(&Action::Redeal(other.redeal));
        result = result.combine(&Action::Cut(other.cut));
        if other.restacked {
            result = result.combine(&Action::Restack);
        }
        result
    }

    fn combine_all<'a, I>(size: usize, actions: I) -> CombinedAction
        where I: IntoIterator<Item = &'a Action> {
        let mut result = CombinedAction::empty(size);
        for action in actions.into_iter() {
            result = result.combine(action);
        }
        return result;
    }
}

fn pp(deck: &Deck) -> String {
    //deck.iter().map(|x| if *x > 9 {(('A' as u8 - 10 + *x as u8) as char).to_string()} else {x.to_string()}).collect()
    deck.iter().position(|x| *x == 0).unwrap().to_string()
}

fn pos(deck: &Deck) -> i64 {
    deck.iter().position(|x| *x == 0).unwrap() as i64
}

/*fn inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;    
    let mut r = n;
    let mut newr = a;    
    println!("t: {} r: {} newt:{} newr:{}", t, r, newt, newr);
    while newr != 0 {
        let quotient = r / newr;
        // (t, newt) = (newt, t - quotient * newt);
        let temp = newt;
        newt = t - quotient * newt;
        t = newt;
        // (r, newr) = (newr, r - quotient * newr);
        let temp = newr;
        newr = r - quotient * newr;
        r = newr;
        println!("q:{} t: {} r: {} newt:{} newr:{}", quotient, t, r, newt, newr);
    }
    if r > 1 {
        panic!("{} is not invertible under {}", a, n);
    }
    if t < 0 {
        t += n;
    }
    println!("Inverse of {} mod {} is {}", a, n, t);
    return t;
}*/

fn inverse(a: i64, n: i64) -> i64 {
    modinverse(a, n).unwrap()
}

fn invert(result: i64, base: i64, size: i64) -> i64 {
    //result = base * x % size;
    // x = result / base % size;
    // x = result * inv(base) % size;
    (result as i128 * inverse(base, size) as i128 % size as i128) as i64
}

fn power(mut square: CombinedAction, mut n: i64) -> CombinedAction {
    let mut result = CombinedAction::empty(square.size as usize);
    while n > 0 {
        let bit = n % 2;
        if bit == 1 {
            result = result.merge(&square);
        }
        square = square.merge(&square);
        n /= 2;
    }
    result
}

fn card_at(ca: &CombinedAction, mut n: i64) -> i64 {
    if ca.restacked {
        n = ca.size - n - 1;
    }
    n += ca.cut;
    // Invert n
    invert(n, ca.redeal as i64, ca.size)
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    let mut deck = deal(10007);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        let action = parse(&line);
        deck = act(deck, &action);
    }
    println!("Position of card 2019 is {:?}", deck.iter().position(|c| *c == 2019));

    println!("{:?}", power(CombinedAction::combine_all(100, &vec![Action::Cut(2)]), 30));

    /*let deck = deal(11);
    for i in 2..deck.len() {
        let mut deck = deck.clone();
        //deck = redeal(deck, 4);
        for j in 2..deck.len() {
            let mut target = cut(deck.clone(), i as i64);
            target = redeal(target, j);

            let actual = redeal(deck.clone(), j);

            let shift = pos(&target) - pos(&actual);

            let calced = i * j % deck.len();

            print!("  {}={}", shift, calced);

            assert_eq!(cut(actual, calced as i64), target);
        }
        println!("");
    }*/

    let mut ca = CombinedAction::empty(119315717514047);
    
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read");
        let action = parse(&line);
        ca = ca.combine(&action);
    }

    let many = power(ca, 101741582076661);

    println!("{:?}", many);

    println!("{}", card_at(&many, 2020));

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_deal() {
        let deck = deal(10);
        assert_eq!(deck.len(), 10);
        assert_eq!(deck[deck.len() - 1], 9);
    }
    
    #[test]
    fn test_restack() {
        let mut deck = deal(10);
        deck = restack(deck);
        assert_eq!(deck[deck.len() - 1], 0);
    }
    
    #[test]
    fn test_cut() {
        let mut deck = deal(10);
        deck = cut(deck, 3);
        assert_eq!(deck[deck.len() - 1], 2);
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = deal(10);
        deck = cut(deck, -3);
        assert_eq!(deck[deck.len() - 1], 6);
    }

    #[test]
    fn test_redeal1() {
        let mut deck = deal(10);
        deck = redeal(deck, 1);
        assert_eq!(deck, deal(10));
    }
    
    #[test]
    fn test_redeal3() {
        let mut deck = deal(10);
        deck = redeal(deck, 3);
        assert_eq!(deck, vec![0,7,4,1,8,5,2,9,6,3]);
    }

    fn test_combined_actions(actions: &Vec<Action>) {
        let deck = deal(11);
        let combined_action = CombinedAction::combine_all(deck.len(), actions);
        let combined_deck = combined_action.execute(deck.clone());
        let one_by_one_deck = execute_all(deck.clone(), actions);
        assert_eq!(combined_deck, one_by_one_deck);
    }

    #[test]
    fn test_combining_restack() {
        let mut actions = vec![Action::Restack, Action::Restack, Action::Restack];
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
    }
    
    #[test]
    fn test_combining_cut() {
        let mut actions = vec![Action::Cut(10), Action::Cut(-5), Action::Cut(3)];
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
    }
    
    #[test]
    fn test_combining_cut_and_restack() {
        let mut actions = vec![Action::Restack, Action::Cut(10), Action::Restack, Action::Cut(-5), Action::Restack, Action::Cut(3)];
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
    }

    #[test]
    fn test_combining_redeal() {
        let mut actions = vec![Action::Redeal(2), Action::Redeal(2), Action::Redeal(3), Action::Redeal(7)];
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
        actions.pop();
        test_combined_actions(&actions);
    }

    fn test_all_actions_seq(mut actions: Vec<Action>) {
        loop {
            test_combined_actions(&actions);
            let result = actions.pop();
            if result.is_none() {
                break;
            }
        }
    }

    #[test]
    fn test_combining_redeal_and_restack() {
        test_all_actions_seq(vec![Action::Redeal(2), Action::Restack, Action::Redeal(2), Action::Restack, Action::Redeal(3), Action::Redeal(7)]);
    }

    #[test]
    fn test_combining_redeal_and_cut() {
        test_all_actions_seq(vec![Action::Redeal(2), Action::Cut(2), Action::Redeal(2), Action::Cut(1), Action::Redeal(3), Action::Cut(-3)]);
    }
    
    #[test]
    fn test_combining_all() {
        test_all_actions_seq(vec![Action::Redeal(2), Action::Cut(2), Action::Restack, Action::Cut(4), Action::Redeal(2), Action::Cut(1), Action::Restack, Action::Redeal(3), Action::Cut(-3)]);
    }
}
