use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    distance: i32
}

impl Point
{
    pub fn new(x: i32, y: i32, distance: i32) -> Point {
        Point { x: x, y: y, distance: distance }
    }
}

fn parse_wire(line: &str) -> Vec<Point> {
    let mut current = Point {x: 0, y: 0, distance: 0};

    return line.split(",").map(|item| {
        let item = item.trim();
        let direction = item.chars().nth(0);
        let distance: i32 = item[1..].parse().expect("Not a number");
        current = match direction {
            Some('R') => Point { x: current.x + distance, distance: current.distance + distance, ..current},
            Some('L') => Point { x: current.x - distance, distance: current.distance + distance, ..current},
            Some('D') => Point { y: current.y + distance, distance: current.distance + distance, ..current},
            Some('U') => Point { y: current.y - distance, distance: current.distance + distance, ..current},
            _ => panic!("Unknown direction {:?}", direction)
        };
        return current.clone();
    }).collect::<Vec<Point>>();
}


#[derive(PartialEq, Debug, Clone)]
struct Line {
    f: Point,
    t: Point,
}

impl Line
{
    pub fn new(f: Point, t: Point) -> Line {
        Line { f: f, t: t }
    }
}

fn intersection(la: &Line, lb: &Line) -> Option<Point> {
    if la.f.x == la.t.x {
        let lax = la.f.x;
        let laymin = cmp::min(la.f.y, la.t.y);
        let laymax = cmp::max(la.f.y, la.t.y);
        if lb.f.y == lb.t.y {
            let lby = lb.f.y;
            let lbxmin = cmp::min(lb.f.x, lb.t.x);
            let lbxmax = cmp::max(lb.f.x, lb.t.x);
            if lbxmin <= lax && lax <= lbxmax && laymin <= lby && lby <= laymax {
                // Intersection at lax, lby
                let ladistance = (la.f.y - lby).abs();
                let lbdistance = (lb.f.x - lax).abs();
                return Some(Point {x: lax, y: lby, distance: la.f.distance + ladistance + lb.f.distance + lbdistance});
            }
        }
        return None;
    } else {
        if lb.f.x == lb.t.x {
            return intersection(lb, la);
        } else {
            return None;
        }
    }
}

fn find_intersections(a_list: &Vec<Point>, b_list: &Vec<Point>) -> Vec<Point> {
    let mut output = Vec::new();

    let mut last_a = Point { x: 0, y: 0, distance: 0 };

    for a in a_list {
        let mut last_b = Point { x: 0, y: 0, distance: 0 };
        for b in b_list {
            let x = intersection(&Line::new(last_a, a.clone()), &Line::new(last_b, b.clone()));
            if let Some(x) = x {
                if x != (Point { x: 0, y: 0, distance: 0 }) {
                    output.push(x);
                }
            }
            last_b = b.clone();
        }
        last_a = a.clone();
    }

    return output;
}

fn nearest_point(p_list: Vec<Point>) -> Option<Point> {
    let mut nearest: Option<Point> = None;

    for p in &p_list {
        if let Some(nearest) = nearest {
            if p.x.abs() + p.y.abs() > nearest.x.abs() + nearest.y.abs() {
                continue;
            }
        }
        nearest = Some(p.clone());
    }

    return nearest;
}

fn nearest_intersection(a_list: &Vec<Point>, b_list: &Vec<Point>) -> Option<Point> {
    return nearest_point(find_intersections(a_list, b_list));
}

fn closest_point(p_list: Vec<Point>) -> Option<Point> {
    let mut nearest: Option<Point> = None;

    for p in &p_list {
        if let Some(nearest) = nearest {
            if p.distance > nearest.distance {
                continue;
            }
        }
        nearest = Some(p.clone());
    }

    return nearest;
}

fn closest_intersection(a_list: &Vec<Point>, b_list: &Vec<Point>) -> Option<Point> {
    return closest_point(find_intersections(a_list, b_list));
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let a_list = parse_wire(&(lines.next().expect("Failed to read").expect("WAT")));
    let b_list = parse_wire(&(lines.next().expect("Failed to read").expect("WAT")));

    let p = nearest_intersection(&a_list, &b_list).expect("No intersections");

    let q = closest_intersection(&a_list, &b_list).expect("No intersections");

    println!("Nearest: {:?} Closest: {:?}", p, q);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

/*    fn test_case(start: &str, expected: &str) {
        let expected = parse(expected);
        let mut actual = parse(start);
        run(&mut actual);
        assert_eq!(actual, expected);
    }*/

    #[test]
    fn test_parse_wire() {
        let points = parse_wire("R8,U5,L5,D3");
        assert_eq!(points[0], Point::new(8, 0, 8));
        assert_eq!(points[1], Point::new(8, -5, 13));
        assert_eq!(points[2], Point::new(3, -5, 18));
        assert_eq!(points[3], Point::new(3, -2, 21));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(
            &Line::new(
                Point::new(0, 0, 5),
                Point::new(0, 10, 15)
            ),
            &Line::new(
                Point::new(15, 3, 80),
                Point::new(-4, 3, 99)
            )),
            Some(Point::new(0,3, 5 + 3 + 80 + 15))
        );
    }

    #[test]
    fn test_find_intersections() {
        let x_list = find_intersections(&parse_wire("R8,U5,L5,D3"), &parse_wire("U7,R6,D4,L4"));
        assert_eq!(x_list[0], Point::new(6, -5, 30));
        assert_eq!(x_list[1], Point::new(3, -3, 40));
    }
    
    #[test]
    fn test_nearest_point() {
        let points = parse_wire("R8,U5,L5,D3");
        assert_eq!(nearest_point(points), Some(Point::new(3, -2, 21)));
    }

    #[test]
    fn test_nearest_intersection() {
        let x = nearest_intersection(&parse_wire("R8,U5,L5,D3"), &parse_wire("U7,R6,D4,L4"));
        assert_eq!(x, Some(Point::new(3, -3, 40)));
    }
    
    #[test]
    fn test_closest_intersection() {
        let x = closest_intersection(&parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), &parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")).unwrap();
        assert_eq!(x.distance, 410);
    }
}
