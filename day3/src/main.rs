extern crate geo;
extern crate line_intersection;

use geo::{Line, Point};

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/*#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}*/

fn parse_wire(line: &str) -> Vec<Point<f32>> {
    let mut current = Point::new(0.0, 0.0);

    return line.split(",").map(|item| {
        let item = item.trim();
        let direction = item.chars().nth(0);
        let distance: f32 = item[1..].parse().expect("Not a number");
        current = match direction {
            Some('R') => *current.set_x(current.x() + distance),
            Some('L') => *current.set_x(current.x() - distance),
            Some('D') => *current.set_y(current.y() + distance),
            Some('U') => *current.set_y(current.y() - distance),
            _ => panic!("Unknown direction {:?}", direction)
        };
        return current.clone();
    }).collect::<Vec<Point<f32>>>();
}

fn intersection(a: &Line<f32>, b: &Line<f32>) -> Option<Point<f32>> {
    use line_intersection::{LineInterval};

    let a = LineInterval::line_segment(a.clone());

    let b = LineInterval::line_segment(b.clone());

    return a.relate(&b).unique_intersection();
}

fn find_intersections(a_list: Vec<Point<f32>>, b_list: Vec<Point<f32>>) -> Vec<Point<f32>> {
    let mut output = Vec::new();

    let mut last_a = Point::new(0.0, 0.0);

    for a in &a_list {
        let mut last_b = Point::new(0.0, 0.0);
        for b in &b_list {
            let x = intersection(&Line::new(last_a, a.clone()), &Line::new(last_b, b.clone()));
            if let Some(x) = x {
                if x != Point::new(0.0, 0.0) {
                    output.push(x);
                }
            }
            last_b = b.clone();
        }
        last_a = a.clone();
    }

    return output;
}

fn nearest_point(p_list: Vec<Point<f32>>) -> Point<f32> {
    let mut nearest = Point::new(std::f32::INFINITY, std::f32::INFINITY);

    for p in &p_list {
        if p.x().abs() + p.y().abs() < nearest.x().abs() + nearest.y().abs() {
            nearest = p.clone();
        }
    }

    return nearest;
}

fn nearest_intersection(a_list: Vec<Point<f32>>, b_list: Vec<Point<f32>>) -> Point<f32> {
    return nearest_point(find_intersections(a_list, b_list));
}

fn main() {
    let file = File::open("input").expect("Failed to open input");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let a_list = parse_wire(&(lines.next().expect("Failed to read").expect("WAT")));
    let b_list = parse_wire(&(lines.next().expect("Failed to read").expect("WAT")));

    let p = nearest_intersection(a_list, b_list);

    println!("x: {} y: {}", p.x(), p.y());

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
        assert_eq!(points[0], Point::new(8.0, 0.0));
        assert_eq!(points[1], Point::new(8.0, -5.0));
        assert_eq!(points[2], Point::new(3.0, -5.0));
        assert_eq!(points[3], Point::new(3.0, -2.0));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(
            &Line::new(
                Point::new(0.0, 0.0),
                Point::new(0.0, 10.0)
            ),
            &Line::new(
                Point::new(15.0, 3.0),
                Point::new(-4.0, 3.0)
            )),
            Some(Point::new(0.0,3.0))
        );
    }

    #[test]
    fn test_find_intersections() {
        let x_list = find_intersections(parse_wire("R8,U5,L5,D3"), parse_wire("U7,R6,D4,L4"));
        assert_eq!(x_list[0], Point::new(6.0, -5.0));
        assert_eq!(x_list[1], Point::new(3.0, -3.0));
    }
    
    #[test]
    fn test_nearest_point() {
        let points = parse_wire("R8,U5,L5,D3");
        assert_eq!(nearest_point(points), Point::new(3.0, -2.0));
    }

    #[test]
    fn test_nearest_intersection() {
        let x = nearest_intersection(parse_wire("R8,U5,L5,D3"), parse_wire("U7,R6,D4,L4"));
        assert_eq!(x, Point::new(3.0, -3.0));
    }
}
