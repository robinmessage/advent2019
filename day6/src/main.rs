use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

#[derive(PartialEq, Debug, Clone)]
struct Orbit {
    name: String,
    parent: String
}

impl Orbit
{
    pub fn new(name: &str, parent: &str) -> Orbit {
        Orbit { name: name.to_string(), parent: parent.to_string() }
    }

    pub fn depth(&self, orbits: &Vec<Orbit>) -> i32 {
        let p = find_orbit(orbits, &self.parent);
        return match p {
            Some(orbit) => 1 + orbit.depth(orbits),
            None => 1
        };
    }
    
    pub fn list<'a>(&'a self, orbits: &'a Vec<Orbit>) -> Vec<&'a str> {
        let mut o = self;
        let mut result = Vec::<&str>::new();
        loop {
            result.push(&o.name);
            let p = find_orbit(orbits, &o.parent);
            match p {
                Some(orbit) => o = orbit,
                None => break
            };
        }
        return result;
    }
}

fn find_orbit<'a>(orbits: &'a Vec<Orbit>, name: &str) -> Option<&'a Orbit> {
    return orbits.iter().find(|orbit| orbit.name == name);
}

fn parse_orbit(line: &str) -> Orbit {
    let parts = line.split(")").collect::<Vec<&str>>();
    return Orbit::new(parts[1], parts[0]);
}

fn parse_orbits(lines: &Vec<&str>) -> Vec<Orbit> {
    return lines.into_iter().map(|orbit| parse_orbit(orbit)).collect::<Vec<Orbit>>();
}

fn count_orbits(orbits: &Vec<Orbit>) -> i32 {
    let mut count = 0;
    for orbit in orbits {
        count += orbit.depth(orbits);
    }

    return count;
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.expect("Failed to read line")).collect::<Vec<String>>();

    let linesRef: Vec<&str> = lines.iter().map(|s| s.as_ref()).collect();

    let orbits = parse_orbits(&linesRef);

    //let checksum = count_orbits(&orbits);

    //println!("Count: {}", checksum);

    let mut san = find_orbit(&orbits, "SAN").expect("Missing orbit").list(&orbits);
    let mut you = find_orbit(&orbits, "YOU").expect("Missing orbit").list(&orbits);
    while san.last() == you.last() {
        san.pop();
        you.pop();
    }
    println!("SAN {} {}", san.join(","), san.len());
    println!("YOU {} {}", you.join(","), you.len());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_orbits() {
        let orbits = parse_orbits(&(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".split("\n").collect::<Vec<&str>>()));
        assert_eq!(orbits[0], Orbit::new("B", "COM"));
        assert_eq!(orbits[10], Orbit::new("L", "K"));
    }
    
    #[test]
    fn test_count_orbits() {
        let orbits = parse_orbits(&(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".split("\n").collect::<Vec<&str>>()));
        assert_eq!(count_orbits(&orbits), 42);
    }

    #[test]
    fn test_list() {
        let orbits = parse_orbits(&(r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".split("\n").collect::<Vec<&str>>()));
        assert_eq!(find_orbit(&orbits, "L").expect("Missing orbit").list(&orbits), vec!["L", "K", "J", "E", "D", "C", "B"]);
    }
}
