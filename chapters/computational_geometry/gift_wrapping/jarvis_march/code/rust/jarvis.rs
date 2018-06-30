use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn cross(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl<'a> Add for &'a Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<'a> Sub for &'a Point {
    type Output = Point;
    
    fn sub(self, other: &'a Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

fn jarvis_cross(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let v1 = p2 - p1;
    let v2 = p3 - p2;
    let ret = Point::cross(v1, v2);
    ret*ret
}

fn main() {
    let p1 = Point { x:  10.0, y:  10.0 };
    let p2 = Point { x: -10.0, y: -10.0 };

    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);

    println!("P1 + P2: {:?}", &p1 + &p2);
    println!("P1 - P2: {:?}", &p1 - &p2);
    println!("P1 x P2: {}", p1.cross(p2));
}
