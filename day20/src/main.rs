use std::fs;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fmt;

#[derive(Debug)]
struct PlainMaze {
    width: i32,
    height: i32,
    map: Vec<char>
}

impl PlainMaze {
    fn get(self: &PlainMaze, x: i32, y: i32) -> char {
        return self.map[(x + y * self.width) as usize];
    }
}

fn parse_maze(maze: &str) -> PlainMaze {
    let mut map = vec![];
    let mut width: i32 = 0;
    let mut height: i32 = 0;

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
                    println!("Odd row, got {}, expected {} ({})", row.len(), width, row.iter().collect::<String>());
                }
            }
            first_row = false;
            row = vec![];
        } else {
            row.push(c);
        }
    }
    let len = row.len() as i32;
    if first_row || len == width {
        height += 1;
        width = len;
        map.append(&mut row);
    }

    return PlainMaze {
        width,
        height,
        map
    };

}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Portal(u32);

impl Portal {
    fn new(c1: char, c2: char) -> Portal {
        let bytes: Vec<u32> = vec![c1, c2].iter().map(|c| (*c as u8 - 'A' as u8) as u32).collect();
        return Portal(bytes[0] * 26 + bytes[1]);
    }

    fn named(name: &str) -> Portal {
        let bytes: Vec<u32> = name.as_bytes().iter().map(|c| (*c as u8 - 'A' as u8) as u32).collect();
        return Portal(bytes[0] * 26 + bytes[1]);
    }

    fn name(self: &Portal) -> String {
        let bytes = vec![self.0 / 26, self.0 % 26];
        return bytes.iter().map(|c| (*c as u8 + 'A' as u8) as char).collect();
    }
}

impl fmt::Debug for Portal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn is_portal(c: char) -> bool {
    return ('A'..='Z').contains(&c);
}

#[derive(Debug)]
struct PortalMaze {
    width: i32,
    height: i32,
    map: Vec<char>,
    portals: HashMap<Portal, Vec<(i32, i32)>>
}

fn insert_portal(portals: &mut HashMap<Portal, Vec<(i32, i32)>>, portal: Portal, at: (i32, i32)) {
    portals.entry(portal).or_insert_with(|| vec![]).push(at);
}

fn find_portals(maze: &PlainMaze) -> HashMap<Portal, Vec<(i32, i32)>> {
    let mut portals = HashMap::new();
    let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 1..(maze.height - 1) {
        for x in 1..(maze.width - 1) {
            let first = maze.get(x, y);
            if is_portal(first) {
                for offset in &offsets {
                    let second = maze.get(x - offset.0, y - offset.1);
                    let opposite = maze.get(x + offset.0, y + offset.1);
                    if is_portal(second) && opposite == '.' {
                        let portal = if offset.0 < 0 || offset.1 < 0 {
                            Portal::new(second, first)
                        } else {
                            Portal::new(first, second)
                        };
                        insert_portal(&mut portals, portal, (x + offset.0, y + offset.1));
                    }
                }
            }
        }
    }

    return portals;
}

impl PortalMaze {
    fn new(maze: PlainMaze) -> PortalMaze {

        let portals = find_portals(&maze);

        return PortalMaze {
            width: maze.width,
            height: maze.height,
            map: maze.map,
            portals
        };
    }

    fn is_inner(&self, location: &(i32, i32)) -> bool {
        location.0 > 2 && location.0 < self.width - 3 && location.1 > 2 && location.1 < self.height - 3
    }
}

#[derive(Debug)]
struct JoinedMaze {
    width: i32,
    height: i32,
    map: Vec<char>,
    entrance: (i32, i32),
    exit: (i32, i32),
    portals: HashMap<Portal, Vec<(i32, i32)>>,
    portal_map: HashMap<(i32, i32), (i32, i32, Portal, bool)>
}

impl JoinedMaze {
    fn new(maze: PortalMaze) -> JoinedMaze {
        let entrance = maze.portals.get(&Portal::named("AA")).unwrap()[0];
        let exit = maze.portals.get(&Portal::named("ZZ")).unwrap()[0];

        let mut portal_map = HashMap::new();

        for (portal, locations) in &maze.portals {
            for i in 0..locations.len() {
                let location = locations[i];
                let other = locations[locations.len() - i - 1]; // Map entrance/exit to itself
                portal_map.insert(location, (other.0, other.1, *portal, maze.is_inner(&location)));
            }
        }

        return JoinedMaze {
            width: maze.width,
            height: maze.height,
            map: maze.map,
            portals: maze.portals,
            entrance,
            exit,
            portal_map
        };
    }
    
    fn get(&self, x: i32, y: i32) -> char {
        return self.map[(x + y * self.width) as usize];
    }
}

fn solve(maze: &JoinedMaze) -> i32 {
    fn try_location(maze: &JoinedMaze, visited: &mut HashSet<(i32, i32)>, queue: &mut VecDeque<((i32, i32), i32)>, location: (i32, i32), distance: i32) {
        if visited.contains(&location) {
            return;
        }
        if maze.get(location.0, location.1) == '.' {
            queue.push_back((location, distance + 1));
        }
    }

    let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    queue.push_back((maze.entrance, 0));

    while let Some((location, distance)) = queue.pop_front() {
        println!("Visit {:?} at {}", location, distance);
        if location == maze.exit {
            return distance;
        }
        visited.insert(location);
        for offset in &offsets {
            let l = (location.0 + offset.0, location.1 + offset.1);
            try_location(maze, &mut visited, &mut queue, l, distance);
        }
        if let Some((x, y, p, inner)) = maze.portal_map.get(&location) {
            try_location(maze, &mut visited, &mut queue, (*x, *y), distance);
            println!("  with portal {:?} inner: {}", p, inner);
        }
    }

    panic!("Couldn't find the way");
}

fn solve_inception(maze: &JoinedMaze) -> i32 {
    fn try_location(maze: &JoinedMaze, visited: &mut HashSet<(i32, i32, i32)>, queue: &mut VecDeque<(i32, i32, i32, i32)>, location: (i32, i32, i32), distance: i32) {
        if visited.contains(&location) {
            return;
        }
        if maze.get(location.0, location.1) == '.' {
            queue.push_back((location.0, location.1, location.2, distance + 1));
        }
    }

    let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut queue: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    queue.push_back((maze.entrance.0, maze.entrance.1, 0, 0));

    while let Some((x, y, depth, distance)) = queue.pop_front() {
        println!("Visit {}, {} at {} ({})", x, y, depth, distance);
        if (x, y) == maze.exit && depth == 0 {
            return distance;
        }
        visited.insert((x, y, depth));
        for offset in &offsets {
            let l = (x + offset.0, y + offset.1, depth);
            try_location(maze, &mut visited, &mut queue, l, distance);
        }
        if let Some((px, py, p, inner)) = maze.portal_map.get(&(x, y)) {
            if *px != x && *py != y { // Ignore self-closed AA and ZZ portals
                println!("  with portal {:?} inner: {}", p, inner);
                let lower = (*px, *py, depth - 1);
                let upper = (*px, *py, depth + 1);
                if *inner {
                    if depth == 0 || !visited.contains(&lower) {
                        try_location(maze, &mut visited, &mut queue, upper, distance);
                    }
                } else {
                    if depth > 0 {
                        try_location(maze, &mut visited, &mut queue, lower, distance);
                    }
                }
            }
        }
    }

    panic!("Couldn't find the way");
}

/*fn try_location(maze: &RoutelessMaze, x: i32, y: i32, queue: &mut VecDeque<((i32, i32), i32, KeySet)>, visited: &HashSet<(i32, i32)>, score: i32, doors: &KeySet) {
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

fn distance_and_keys(maze: &RoutelessMaze, a: char, b: char) -> Option<(i32, KeySet)> {
    // Walk the goddammed maze breadth-first
    let mut queue = VecDeque::<((i32, i32), i32, KeySet)>::new();
    let mut visited = HashSet::new();

    let location = find_in(maze, a);

    queue.push_back((location, 0, KeySet::new()));

    while queue.len() > 0 {
        let (location, score, doors) = queue.pop_front().unwrap();
        if get_pixel(maze, location) == b {
            println!("Found {} to {}, distance {}, doors {:?}", a, b, score, doors);
            return Some((score, doors));
        }
        visited.insert(location);
        try_location(maze, location.0 - 1, location.1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0 + 1, location.1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0, location.1 - 1, &mut queue, &visited, score, &doors);
        try_location(maze, location.0, location.1 + 1, &mut queue, &visited, score, &doors);
    }

    println!("Can't get from {} to {}", a, b);
    return None;
}

fn index_to_key(x: usize) -> char {
    return ('a' as u8 + x as u8) as char;
}

fn key_to_index(k: char) -> usize {
    return (k as u8 - 'a' as u8) as usize;
}

fn index_to_start(x: usize) -> char {
    return ('0' as u8 + x as u8) as char;
}

#[derive(Debug)]
struct Maze {
    width: i32,
    height: i32,
    map: Vec<char>,
    keys: Vec<(i32, i32)>,
    doors: Vec<(i32, i32)>,
    starts: Vec<(i32, i32)>,
    routes: HashMap<(char, char), (i32, KeySet)>,
    minimums: HashMap<char, i32>
}

impl Maze {
    pub fn new(maze: RoutelessMaze) -> Maze {
        let key_count = maze.keys.len();

        let mut routes = HashMap::new();
        let mut minimums = HashMap::new();

        let count = key_count + maze.starts.len();

        for a_i in 0..count {
            let a = if a_i >= key_count {index_to_start(a_i - key_count)} else {index_to_key(a_i)};
            let mut min = 1_000_000_000;
            for b in 0..count {
                let b = if b >= key_count {index_to_start(b - key_count)} else {index_to_key(b)};
                if a == b {
                    continue;
                }
                let possible = distance_and_keys(&maze, a, b);
                if let Some((distance, keys_needed)) = possible {
                    routes.insert((a, b), (distance, keys_needed));
                    if distance < min {
                        min = distance;
                    }
                }
            }
            if a_i < key_count {
                minimums.insert(a, min);
            }
        }

        return Maze {
            width: maze.width,
            height: maze.height,
            map: maze.map,
            keys: maze.keys,
            doors: maze.doors,
            starts: maze.starts,
            routes,
            minimums
        };
    }

    fn distance(self: &Maze, a: char, b: char) -> Option<i32> {
        if let Some((distance, _)) = self.routes.get(&(a, b)) {
            return Some(*distance);
        } else {
            return None;
        }
    }
    
    fn keys_needed(self: &Maze, a: char, b: char) -> Option<&KeySet> {
        if let Some((_, keys_needed)) = self.routes.get(&(a, b)) {
            return Some(keys_needed);
        } else {
            return None;
        }
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
        if let Some(keys_needed) = maze.keys_needed(location, index_to_key(*key)) {
            keys_needed.intersection(keys).len() == keys_needed.len()
        } else {
            false
        }
    });
}



#[derive(Clone, Eq, PartialEq)]
struct State {
    estimate: i32,
    distance: i32,
    locations: Vec<char>,
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
            .then_with(|| self.keys.0.cmp(&other.keys.0))
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
    let mut bests: HashMap<(Vec<char>, KeySet), i32> = HashMap::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {distance: 0, estimate: 0, locations: (0..(maze.starts.len())).map(|c| ('0' as u8 + c as u8) as char).collect(), keys: KeySet::new()});

    let mut best = 0;

    while let Some(State { distance, estimate, locations, keys }) = heap.pop() {
        if estimate > best {
            println!("At {:?} distance {} with estimate {} keys {:?}", locations, distance, estimate, keys);
            best = estimate;
        }
        if keys.len() == maze.keys.len() {
            // Finished!
            return distance;
        }

        for i in 0..locations.len() {
            let location = locations[i];

            let possible_keys = accessible_keys(maze, location, &keys);
            for key_index in 0..32 {
                if !possible_keys.contains(key_index) {
                    continue;
                }
                let key = index_to_key(key_index);
                if let Some(d) = maze.distance(location, key) {
                    let new_distance = distance + d;
                    let new_keys = keys.insert(key_index);

                    let e = new_distance + maze.estimate(&new_keys);

                    let mut new_locations = locations.clone();
                    new_locations[i] = key;

                    if let Some(current_d) = bests.get(&(new_locations.clone(), new_keys)) {
                        if new_distance >= *current_d {
                            continue;
                        }
                    }
                    bests.insert((new_locations.clone(), new_keys), new_distance);
                    heap.push(State {distance: new_distance, estimate: e, locations: new_locations, keys: new_keys});
                }
            }
        }
    }

    panic!("Queue empty but goal not found");
}*/

/*fn graph_it(maze: &Maze) {
    println!("digraph G{{");
    let key_count = maze.keys.len();
    for a_index in 0..=key_count {
        let a = if a_index == key_count {'@'} else {index_to_key(a_index)};
        for b in (a_index + 1)..=key_count {
            let b = if b == key_count {'@'} else {index_to_key(b)};
            if let Some(distance) = maze.distance(a, b) {
                println!("{} -> {} [label=\"{} {:?}\"]", a, b, distance , maze.keys_needed(a, b).unwrap());
            }
        }
    }
    println!("}}");
}*/

fn main() {
    let maze = JoinedMaze::new(PortalMaze::new(parse_maze(&fs::read_to_string("input").expect("Couldn't read input"))));

    println!("{:#?}", maze);
    println!("{}", solve(&maze));
    println!("{}", solve_inception(&maze));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_maze_parse() {
        let maze = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";
        let maze = JoinedMaze::new(PortalMaze::new(parse_maze(maze)));

        println!("{:#?}", maze);
    }

    #[test]
    fn test_maze_solve1() {
        let maze = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";
        let maze = JoinedMaze::new(PortalMaze::new(parse_maze(maze)));

        assert_eq!(solve(&maze), 23);
    }

    #[test]
    fn test_maze_solve2() {
        let maze = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";
        let maze = JoinedMaze::new(PortalMaze::new(parse_maze(maze)));

        assert_eq!(solve_inception(&maze), 26);
    }


    #[test]
    fn test_maze_solve3() {
        let maze = r"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
";
        let maze = JoinedMaze::new(PortalMaze::new(parse_maze(maze)));

        assert_eq!(solve_inception(&maze), 396);
    }
}
