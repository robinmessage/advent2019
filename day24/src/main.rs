use std::fs;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct World {
    life: Vec<bool>,
    width: i32,
    height: i32
}

fn parse_world(world: &str) -> World {
    let mut life = vec![];
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let mut first_row = true;
    let mut row = vec![];

    for c in world.chars() {
        if c == '\n' { // New line
            let len = row.len() as i32;
            if first_row || len == width {
                height += 1;
                width = len;
                life.append(&mut row);
            } else {
                if row.len() > 0 {
                    println!("Odd row, got {}, expected {} ({:?})", row.len(), width, row.iter().collect::<Vec<&bool>>());
                }
            }
            first_row = false;
            row = vec![];
        } else {
            row.push(c == '#');
        }
    }
    let len = row.len() as i32;
    if first_row || len == width {
        height += 1;
        width = len;
        life.append(&mut row);
    }

    return World {
        width,
        height,
        life
    };
}

impl World {
    fn empty() -> World {
        World {
            width: 5,
            height: 5,
            life: (0..25).map(|_| false).collect()
        }
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            0
        } else {
            if self.life[(y * self.width + x) as usize] {1} else {0}
        }
    }

    fn step(&self) -> World {
        let mut life = Vec::with_capacity(self.life.len());

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbour_count = self.get(x - 1, y) + self.get(x + 1, y) + self.get(x, y - 1) + self.get(x, y + 1);
                let new = neighbour_count == 1 || self.get(x, y) == 0 && neighbour_count == 2;
                life.push(new);
            }
        }

        World {
            life,
            width: self.width,
            height: self.height
        }
    }

    fn biodiversity(&self) -> i32 {
        let mut total = 0;
        for (i, alive) in self.life.iter().enumerate() {
            total |= if *alive {1 << i} else {0};
        }
        total
    }

    fn get_recursive(&self, sx: i32, sy: i32, offset_x: i32, offset_y: i32, below: &World, above: &World) -> i32 {
        let x = sx + offset_x;
        let y = sy + offset_y;
        if x == 2 && y == 2 {
            // Select 5 tiles based on sx and sy
            let which: Vec<usize> = if sx == 1 && sy == 2 {
                vec![0, 5, 10, 15, 20]
            } else if sx == 3 && sy == 2 {
                vec![4, 9, 14, 19, 24]
            } else if sx == 2 && sy == 1 {
                vec![0, 1, 2, 3, 4]
            } else if sx == 2 && sy == 3 {
                vec![20, 21, 22, 23, 24]
            } else {
                panic!("Got sx, sy of {},{}", sx, sy);
            };
            which.iter().map(|x| if below.life[*x] {1} else {0}).sum()
        } else if x < 0 {
            above.get(1, 2)
        } else if x >= self.width {
            above.get(3, 2)
        } else if y < 0 {
            above.get(2, 1)
        } else if y >= self.height {
            above.get(2, 3)
        } else {
            if self.life[(y * self.width + x) as usize] {1} else {0}
        }
    }

    fn step_recursive(&self, below: &World, above: &World) -> World {
        let mut life = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                if x == 2 && y == 2 {
                    life.push(false);
                    continue; // Centre does nothing
                }
                let neighbour_count = self.get_recursive(x, y, -1, 0, below, above)
                                    + self.get_recursive(x, y, 1, 0, below, above)
                                    + self.get_recursive(x, y, 0, -1, below, above)
                                    + self.get_recursive(x, y, 0, 1, below, above);

                //println!("{}, {}: {}", x, y, neighbour_count);
                let new = neighbour_count == 1 || self.get(x, y) == 0 && neighbour_count == 2;
                life.push(new);
            }
        }

        World {
            life,
            width: self.width,
            height: self.height
        }
    }
}

fn loop_until_dup(start: &World) -> i32 {
    let mut seen = HashSet::new();
    let mut current = start.clone();
    loop {
        let next = current.step();
        seen.insert(current);
        if seen.contains(&next) {
            return next.biodiversity();
        }
        current = next;
    }
}

fn step_worlds(inputs: &Vec<World>) -> Vec<World> {
    let mut outputs: Vec<World> = Vec::new();
    let empty = World::empty();
    let len = inputs.len() as i32;
    for i in -1..(len + 1) {
        let level = if i >= 0 && i < len {&inputs[i as usize]} else {&empty};
        let below = if i > 0 {&inputs[(i - 1) as usize]} else {&empty};
        let above = if i < len - 1 {&inputs[(i + 1) as usize]} else {&empty};
        //println!("Calculating level {} ({:?} below:{:?} above:{:?})", i, level, below, above);
        outputs.push(level.step_recursive(below, above));
    }
    outputs
}

fn gold(start: &World, minutes: i32) -> i64 {
    let mut levels: Vec<World> = vec![start.clone()];
    for i in 1..=minutes {
        levels = step_worlds(&levels);
    }
    levels.iter().map(|world| world.life.iter().map(|x| if *x {1} else {0}).sum::<i64>()).sum()
}

fn print_world(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            print!("{}", if world.get(x, y) == 1 {'#'} else {'.'});
        }
        println!("");
    }
}


fn main() {
    let world = parse_world(&fs::read_to_string("input").expect("Couldn't read input"));

    println!("{}", loop_until_dup(&world));
    println!("{}", gold(&world, 200));
    let worlds = step_worlds(&vec![world]);
    println!("Below");
    print_world(&worlds[0]);
    println!("This");
    print_world(&worlds[1]);
    println!("Above");
    print_world(&worlds[2]);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_step() {
        let world = parse_world(r"....#
#..#.
#..##
..#..
#....");
        assert_eq!(world.step(), parse_world(r"#..#.
####.
###.#
##.##
.##.."));
    }

    #[test]
    fn test_biodiversity() {
        let world = parse_world(r".....
.....
.....
#....
.#...");
        assert_eq!(world.biodiversity(), 2129920);
    }
}
