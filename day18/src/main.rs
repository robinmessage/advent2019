use std::fs;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::iter::Iterator;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct KeySet(u32);

impl KeySet {
    fn new() -> KeySet {
        return KeySet(0);
    }
    fn insert(self: &KeySet, index: usize) -> KeySet {
        return KeySet(self.0 | (1 << index));
    }
    fn contains(self: &KeySet, index: usize) -> bool {
        return (self.0 & (1 << index)) != 0;
    }
    fn intersection(self: &KeySet, other: &KeySet) -> KeySet {
        return KeySet(self.0 & other.0);
    }
    fn len(self: &KeySet) -> usize {
        return (0..32).map(|i| if self.contains(i) {1} else {0}).sum();
    }
    fn retain<F>(self: &KeySet, mut predicate: F) -> KeySet 
        where F: FnMut(&usize) -> bool
    {
        let mut result = KeySet::new();
        for x in 0..32 {
            if self.contains(x) && predicate(&x) {
                result = result.insert(x);
            }
        }
        return result;
    }
}

#[derive(Debug)]
struct RoutelessMaze {
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

fn parse_maze(maze: &str) -> RoutelessMaze {
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

    return RoutelessMaze {
        width,
        height,
        map,
        keys,
        doors,
        start
    };
}

fn get_pixel(maze: &RoutelessMaze, (x, y): (i32, i32)) -> char {
    return maze.map[(x + y * maze.width) as usize];
}

fn try_location(maze: &RoutelessMaze, x: i32, y: i32, queue: &mut VecDeque<((i32, i32), i32, KeySet)>, visited: &HashSet<(i32, i32)>, score: i32, doors: &KeySet) {
    if x < 0 || y < 0 || x >= maze.width || y >= maze.height || visited.contains(&(x, y)) {
        return;
    }

    let p = get_pixel(maze, (x, y));

    if p == '#' {
        return
    }

    let mut new_doors = doors.clone();
    if ('A'..='Z').contains(&p) {
        new_doors = new_doors.insert((p as u8 - 'A' as u8) as usize);
    }

    queue.push_back(((x,y), score + 1, new_doors));
}

fn find_in(maze: &RoutelessMaze, a: char) -> (i32, i32) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            if get_pixel(maze, (x, y)) == a {
                return (x, y);
            }
        }
    }
    panic!("Missing {}", a);
}

fn distance_and_keys(maze: &RoutelessMaze, a: char, b: char) -> (i32, KeySet) {
    // Walk the goddammed maze breadth-first
    let mut queue = VecDeque::<((i32, i32), i32, KeySet)>::new();
    let mut visited = HashSet::new();

    let location = find_in(maze, a);

    queue.push_back((location, 0, KeySet::new()));

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

fn index_to_key(x: usize) -> char {
    return ('a' as u8 + x as u8) as char;
}

fn key_to_index(k: char) -> usize {
    return (k as u8 - 'a' as u8) as usize;
}

#[derive(Debug)]
struct Maze {
    width: i32,
    height: i32,
    map: Vec<char>,
    keys: Vec<(i32, i32)>,
    doors: Vec<(i32, i32)>,
    start: (i32, i32),
    routes: HashMap<(char, char), (i32, KeySet)>,
    minimums: HashMap<char, i32>
}

impl Maze {
    pub fn new(maze: RoutelessMaze) -> Maze {
        let key_count = maze.keys.len();

        let mut routes = HashMap::new();
        let mut minimums = HashMap::new();

        for a in 0..=key_count {
            let a = if a == key_count {'@'} else {index_to_key(a)};
            let mut min = 1_000_000_000;
            for b in 0..=key_count {
                let b = if b == key_count {'@'} else {index_to_key(b)};
                if a == b {
                    continue;
                }
                let (distance, keys_needed) = distance_and_keys(&maze, a, b);
                routes.insert((a, b), (distance, keys_needed));
                if distance < min {
                    min = distance;
                }
            }
            if a != '@' {
                minimums.insert(a, min);
            }
        }

        return Maze {
            width: maze.width,
            height: maze.height,
            map: maze.map,
            keys: maze.keys,
            doors: maze.doors,
            start: maze.start,
            routes,
            minimums
        };
    }

    fn distance(self: &Maze, a: char, b: char) -> i32 {
        let (distance, _) = self.routes.get(&(a, b)).unwrap();
        return *distance;
    }
    
    fn keys_needed(self: &Maze, a: char, b: char) -> &KeySet {
        let (_, keys_needed) = self.routes.get(&(a, b)).unwrap();
        return keys_needed;
    }

    fn estimate(self: &Maze, got_keys: &KeySet) -> i32 {
        return self.minimums.iter().map(|(k, e)| if got_keys.contains(key_to_index(*k)) {0} else {*e}).sum(); 
    }
}


fn accessible_keys(maze: &Maze, location: char, keys: &KeySet) -> KeySet {
    // List all the keys we can reach from here
    let mut wanted_keys = KeySet::new();
    for i in 0..maze.keys.len() {
        if !keys.contains(i) {
            wanted_keys = wanted_keys.insert(i);
        }
    }
    return wanted_keys.retain(|key| {
        let keys_needed = maze.keys_needed(location, index_to_key(*key));
        return keys_needed.intersection(keys).len() == keys_needed.len();
    });
}


fn _shortest(maze: &Maze, location: char, keys: &KeySet) -> i32 {
    if keys.len() == maze.keys.len() {
        // Finished!
        return 0;
    }
    let mut min = 1_000_000_000;
    let mut min_key = '?';
    let possible_keys = accessible_keys(maze, location, keys);
    for key_index in 0..32 {
        if !possible_keys.contains(key_index) {
            continue;
        }
        let key = index_to_key(key_index);
        let d = maze.distance(location, key);
        let new_keys = keys.insert(key_index);
        let e = _shortest(maze, key, &new_keys);
        if d + e < min {
            min = d + e;
            min_key = key;
        }
    }
    println!("At {}, possible_keys = {:?}, best = {} for {}", location, &possible_keys, min_key, min);
    return min;
}

fn find_shortest_route(maze: &Maze) -> i32 {
    // Do a graph search from the start
    return _shortest(maze, '@', &KeySet::new());
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    estimate: i32,
    distance: i32,
    location: char,
    keys: KeySet
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.estimate.cmp(&self.estimate)
            .then_with(|| self.distance.cmp(&other.distance))
            .then_with(|| self.location.cmp(&other.location))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_astar(maze: &Maze) -> i32 {
    // Do A*
    let mut bests: HashMap<(char, KeySet), i32> = HashMap::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {distance: 0, estimate: 0, location: '@', keys: KeySet::new()});

    let mut best = 0;

    while let Some(State { distance, estimate, location, keys }) = heap.pop() {
        if estimate > best {
            println!("At {} distance {} with estimate {} keys {:?}", location, distance, estimate, keys);
            best = estimate;
        }
        if keys.len() == maze.keys.len() {
            // Finished!
            return distance;
        }
        let possible_keys = accessible_keys(maze, location, &keys);
        for key_index in 0..32 {
            if !possible_keys.contains(key_index) {
                continue;
            }
            let key = index_to_key(key_index);
            let d = distance + maze.distance(location, key);
            let new_keys = keys.insert(key_index);

            let e = maze.estimate(&new_keys);

            if let Some(current_d) = bests.get(&(key, new_keys)) {
                if d >= *current_d {
                    continue;
                }
            }
            bests.insert((key, new_keys), d);
            heap.push(State {distance: d, estimate: d + e, location: key, keys: new_keys});
        }
    }

    panic!("Queue empty but goal not found");
}

fn graph_it(maze: &Maze) {
    println!("digraph G{{");
    let key_count = maze.keys.len();
    for a_index in 0..=key_count {
        let a = if a_index == key_count {'@'} else {index_to_key(a_index)};
        for b in (a_index + 1)..=key_count {
            let b = if b == key_count {'@'} else {index_to_key(b)};
            println!("{} -> {} [label=\"{} {:?}\"]", a, b, maze.distance(a, b), maze.keys_needed(a, b));
        }
    }
    println!("}}");
}

fn main() {
    let maze = Maze::new(parse_maze(fs::read_to_string("input").expect("Couldn't read input").trim()));

    //graph_it(&maze);
    println!("{}", find_shortest_astar(&maze));
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
        let maze = Maze::new(parse_maze(maze));

        println!("{:?}", maze);
    }
    
    #[test]
    fn test_maze_solve1() {
        let maze = r"#########
#b.A.@.a#
#########";
        let maze = Maze::new(parse_maze(maze));

        println!("{:?}", maze);
        assert_eq!(find_shortest_astar(&maze), 8);
    }

    #[test]
    fn test_maze_solve2() {
        let maze = r"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let maze = Maze::new(parse_maze(maze));

        println!("{:?}", maze);
        //assert_eq!(find_shortest_route(&maze), 86);
        assert_eq!(find_shortest_astar(&maze), 86);
    }

    #[test]
    fn test_maze_solve3() {
        let maze = r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let maze = Maze::new(parse_maze(maze));

        println!("{:?}", maze);
        println!("{}", find_shortest_astar(&maze));
    }

    #[test]
    fn test_keyset() {
        let a = KeySet::new();
        let b = a.insert(1);
        let c = b.insert(4);
        let d = c.retain(|k| *k == 1);
        println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    }
}
