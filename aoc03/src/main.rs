use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

type PointMap = HashMap<Point, i32>;
type PointSet = HashSet<Point>;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let wires: Vec<Vec<&str>> = input
        .trim()
        .split("\n")
        .map(|w| w.split(",").collect())
        .collect();

    let points1 = get_points(&wires[0]);
    let points2 = get_points(&wires[1]);

    part1(&points1, &points2);
    part2(&points1, &points2);
}

fn part1(points1: &PointMap, points2: &PointMap) {
    let set1: PointSet = points1.keys().cloned().collect();
    let set2: PointSet = points2.keys().cloned().collect();
    let distance = set1
        .intersection(&set2)
        .map(|point| point.x.abs() + point.y.abs())
        .min();

    match distance {
        Some(d) => println!("{}", d),
        None => {}
    }
}

fn part2(points1: &PointMap, points2: &PointMap) {
    let set1: PointSet = points1.keys().cloned().collect();
    let set2: PointSet = points2.keys().cloned().collect();
    let distance = set1
        .intersection(&set2)
        .map(|point| points1[point] + points2[point])
        .min();

    match distance {
        Some(d) => println!("{}", d),
        None => {}
    }
}

fn get_points(wire: &Vec<&str>) -> PointMap {
    let mut points = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    for cmd in wire.iter() {
        let (direction, distance) = cmd.split_at(1);

        for _ in 0..distance.parse().unwrap() {
            let (dx, dy) = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _  => (0, 0)
            };

            x += dx;
            y += dy;
            steps += 1;

            points.insert(Point { x, y }, steps);
        }
    }

    points
}
