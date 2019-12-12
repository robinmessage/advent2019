use std::fs;
use std::collections::HashSet;

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl Field {
    pub fn new(input: &str) -> Field {
        let rows = input.split("\n").collect::<Vec<&str>>();
        let width = rows[0].len();
        let height = rows.len();
        return Field { width, height, data: rows.iter().flat_map(|row| row.chars().map(|p|
            match p { '#' => 1, '.' => 0, _ => panic!("Unexpected pixel {}", p) }
        )).collect() };
    }

    pub fn get_pixel(self:&Field, x: usize, y: usize) -> u8 {
        return self.data[x + y * self.width];
    }
}

fn visible_asteroids_from(field: &Field, sx: usize, sy: usize) -> usize {
    let mut angles = HashSet::new();
    // Run through the asteroids
    for ay in 0..field.height {
        for ax in 0..field.width {
            if ax == sx && ay == sy {
                // Can't see asteroid you are on top of
                continue;
            }
            if field.get_pixel(ax, ay) == 0 {
                // No asteroid here
                continue;
            }
            // Calculate angle for each one
            let angle = (ay as f64 - sy as f64).atan2(ax as f64 - sx as f64);
            angles.insert(OrderedFloat(angle));
        }
    }
    // Count unique angles
    return angles.len();
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point(usize, usize);

fn find_best(field: &Field) -> Point {
    let mut best = Point(0, 0);
    let mut best_score = 0;
    for ay in 0..field.height {
        for ax in 0..field.width {
            if field.get_pixel(ax, ay) == 0 {
                // No asteroid to build station here
                continue;
            }
            let score = visible_asteroids_from(field, ax, ay);
            if score > best_score {
                best_score = score;
                best = Point(ax, ay);
            }
        }
    }
    return best;
}

const PI_TIMES_2: f64 = 2.0 * std::f64::consts::PI;

fn map_visible_asteroids_from(field: &Field, sx: usize, sy: usize) -> BTreeMap<OrderedFloat<f64>, BTreeMap<OrderedFloat<f64>, Point>> {
    let mut roids = BTreeMap::new();
    // Run through the asteroids
    for ay in 0..field.height {
        for ax in 0..field.width {
            if ax == sx && ay == sy {
                // Can't see asteroid you are on top of
                continue;
            }
            if field.get_pixel(ax, ay) == 0 {
                // No asteroid here
                continue;
            }
            // Calculate angle and distance for each one
            let y = ay as f64 - sy as f64;
            let x = ax as f64 - sx as f64;
            let mut angle = (y.atan2(x)) + std::f64::consts::FRAC_PI_2;
            if angle < 0.0 {
                angle += PI_TIMES_2;
            }
            let d2 = OrderedFloat(x*x + y*y);
            let target_map = roids.entry(OrderedFloat(angle)).or_insert_with(|| BTreeMap::new());
            target_map.insert(d2, Point(ax, ay));
        }
    }
    return roids;
}


fn vaporised_at(field: &Field, x:usize, y: usize, when: usize) -> Option<Point> {
    let mut roids = map_visible_asteroids_from(field, x, y);
    let mut found = 0;
    let mut any = true;
    while any {
        any = false;
        for (angle, target_map) in roids.iter_mut() {
            if target_map.len() == 0 {
                continue;
            }
            let (target_key, target_value) = target_map.iter_mut().next()?;
            println!("Target angle {} distance {} point {:#?}", angle, target_key.sqrt(), target_value);
            any = true;
            found += 1;
            if found == when {
                return Some(*target_value);
            } else {
                let key_copy = *target_key;
                target_map.remove(&key_copy);
            }
        }
    }
    return None;
}

fn main() {
    let image = Field::new(&fs::read_to_string("input").expect("Couldn't read input").trim());

/*    let mut minimumLayer = 0;
    let mut minimumZeros = image.height * image.width;
    for l in 0..image.layers {
        let mut zeros = 0;
        for y in 0..image.height {
            for x in 0..image.width {
                zeros += if image.get_pixel(x, y, l) == 0 {1} else {0};
            }
        }
        if zeros < minimumZeros {
            minimumZeros = zeros;
            minimumLayer = l;
        }
    }
    let mut ones = 0;
    let mut twos = 0;
    for y in 0..image.height {
        for x in 0..image.width {
            ones += if image.get_pixel(x, y, minimumLayer) == 1 {1} else {0};
            twos += if image.get_pixel(x, y, minimumLayer) == 2 {1} else {0};
        }
    }
    println!("{}", ones * twos);

    for y in 0..image.height {
        println!("{}", (0..image.width).map(|x| 
            if image.get(x, y) == 1 {"*"} else {" "}
        ).collect::<Vec<&str>>().join(""));
    }*/

    let best = find_best(&image);

    println!("{:#?}", best);

    let score = visible_asteroids_from(&image, best.0, best.1);

    println!("{}", score);
    
    let two_hundredth = vaporised_at(&image, best.0, best.1, 200);

    println!("{:#?}", two_hundredth);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_visible_from() {
        let field = Field::new(r".#..#
.....
#####
....#
...##");
        assert_eq!(visible_asteroids_from(&field, 3, 4), 8);
        assert_eq!(visible_asteroids_from(&field, 0, 2), 6);
        assert_eq!(visible_asteroids_from(&field, 4, 2), 5);
    }
    
    #[test]
    fn test_best() {
        let field = Field::new(r".#..#
.....
#####
....#
...##");
        assert_eq!(find_best(&field), Point(3, 4));
    }

    #[test]
    fn test_vaporised_at() {
        let field = Field::new(r".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##");
        assert_eq!(vaporised_at(&field, 8, 3, 36), Some(Point(14, 3)));
    }
    
    #[test]
    fn test_simple_vaporised_at() {
        let field = Field::new(r"###
###
###");
        assert_eq!(vaporised_at(&field, 1, 1, 8), Some(Point(0, 0)));
    }
}
