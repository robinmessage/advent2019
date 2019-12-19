use std::fs;

use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug)]
struct Maze {
    width: i32,
    height: i32,
    map: Vec<char>,
    keys: Vec<(i32, i32)>,
    doors: Vec<(i32, i32)>,
    start: (i32, i32)
}

fn add_vec(vec: &mut Vec<(i32, i32)>, target: u8, value: (i32, i32)) {
    let index = target as usize;
    if vec.len() <= index {
        vec.resize_with(index + 1, || (0, 0));
    }
    vec[index] = value;
}

fn parse_maze(maze: &str) -> Maze {
    let mut map = vec![];
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut keys = vec![];
    let mut doors = vec![];
    let mut start = (0, 0);

    let mut first_row = true;
    let mut row = vec![];

    for c in maze.chars() {
        if c == '\n' { // New line
            let len = row.len() as i32;
            if first_row || len == width {
                height += 1;
                width = len;
                map.append(&mut row);
            } else {
                if row.len() > 0 {
                    println!("Spare {}", row.iter().collect::<String>());
                }
            }
            first_row = false;
            row = vec![];
        } else {
            row.push(c);
            let x = (row.len() - 1) as i32;
            let y = height;
            match c {
                '@' => {start = (x, y)},
                'a'..='z' => {add_vec(&mut keys, c as u8 - 'a' as u8, (x, y))},
                'A'..='Z' => {add_vec(&mut doors, c as u8 - 'A' as u8, (x, y))},
                _ => {}
            }
        }
    }

    return Maze {
        width,
        height,
        map,
        keys,
        doors,
        start
    };
}

fn get_pixel(maze: &Maze, (x, y): (i32, i32)) -> char {
    return maze.map[(x + y * maze.width) as usize];
}

fn try_location(maze: &Maze, x: i32, y: i32, queue: &mut VecDeque<((i32, i32), i32, HashSet<usize>)>, visited: &HashSet<(i32, i32)>, score: i32, doors: &HashSet<usize>) {
    if x < 0 || y < 0 || x >= maze.width || y >= maze.height || visited.contains(&(x, y)) {
        return;
    }

    let p = get_pixel(maze, (x, y));

    if p == '#' {
        return
    }

    let mut new_doors = doors.clone();
    if ('A'..='Z').contains(&p) {
        new_doors.insert((p as u8 - 'A' as u8) as usize);
    }

    queue.push_back(((x,y), score + 1, new_doors));
}

fn find_in(maze: &Maze, a: char) -> (i32, i32) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            if get_pixel(maze, (x, y)) == a {
                return (x, y);
            }
        }
    }
    panic!("Missing {}", a);
}

fn distance_and_keys(maze: &Maze, a: char, b: char) -> (i32, HashSet<usize>) {
    // Walk the goddammed maze breadth-first
    let mut queue = VecDeque::<((i32, i32), i32, HashSet<usize>)>::new();
    let mut visited = HashSet::new();

    let location = find_in(maze, a);

    queue.push_back((location, 0, HashSet::new()));

    while queue.len() > 0 {
        let (location, score, doors) = queue.pop_front().unwrap();
        if get_pixel(maze, location) == b {
            println!("Found {} to {}, distance {}, doors {:?}", a, b, score, doors);
            return (score, doors);
        }
        visited.insert(location);
        try_location(maze, location.0 - 1, location.1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0 + 1, location.1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0, location.1 - 1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0, location.1 + 1, &mut queue, &visited, score, &doors);
    }

    panic!("Can't get from {} to {}", a, b);
}

fn distance_ignoring_keys(maze: &Maze, a: char, b: char) -> i32 {
    let (distance, _) = distance_and_keys(maze, a, b);
    return distance;
}

fn index_to_char(x: usize) -> char {
    return ('a' as u8 + x as u8) as char;
}

fn accessible_keys(maze: &Maze, location: char, keys: &HashSet<usize>) -> HashSet<usize> {
    // List all the keys we can reach from here
    let mut wanted_keys = HashSet::new();
    for i in 0..maze.keys.len() {
        if !keys.contains(&i) {
            wanted_keys.insert(i);
        }
    }
    wanted_keys.retain(|key| {
        let (_, keys_needed) = distance_and_keys(maze, location, index_to_char(*key));
        return keys_needed.intersection(keys).collect::<Vec<&usize>>().len() == keys_needed.len();
    });
    return wanted_keys;
}


fn _shortest(maze: &Maze, location: char, keys: &HashSet<usize>) -> i32 {
    if keys.len() == maze.keys.len() {
        // Finished!
        return 0;
    }
    let mut min = 1_000_000_000;
    let possible_keys = accessible_keys(maze, location, keys);
    println!("At {}, possible_keys = {:#?}", location, &possible_keys);
    for key in possible_keys {
        let d = distance_ignoring_keys(maze, location, index_to_char(key));
        let mut new_keys = keys.clone();
        new_keys.insert(key);
        let e = _shortest(maze, index_to_char(key), &new_keys);
        if d + e < min {
            min = d + e;
        }
    }
    return min;
}

fn find_shortest_route(maze: &Maze) -> i32 {
    // Do a graph search from the start
    return _shortest(maze, '@', &HashSet::new());
}

fn main() {
    let maze = parse_maze(fs::read_to_string("input").expect("Couldn't read input").trim());

    println!("{}", find_shortest_route(&maze));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_maze_parse() {
        let maze = r"#########
#b.A.@.a#
#########";
        let maze = parse_maze(maze);

        println!("{:#?}", maze);
    }
    
    #[test]
    fn test_maze_solve() {
        let maze = r"#########
#b.A.@.a#
#########";
        let maze = parse_maze(maze);

        println!("{:#?}", maze);
        println!("{}", find_shortest_route(&maze));
        assert_eq!(find_shortest_route(&maze), 8);
    }

    #[test]
    fn test_maze_bigger_solve() {
        let maze = r"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let maze = parse_maze(maze);

        println!("{:#?}", maze);
        println!("{}", find_shortest_route(&maze));
        assert_eq!(find_shortest_route(&maze), 86);
    }

    #[test]
    fn test_maze_bigger_solve1() {
        let maze = r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let maze = parse_maze(maze);

        println!("{:#?}", maze);
        println!("{}", find_shortest_route(&maze));
        assert_eq!(find_shortest_route(&maze), 86);
    }

}
