use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl<'a> Sub for &'a Point {
    type Output = Point;
    
    fn sub(self, other: &'a Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

fn is_left_of(a: &Point, b: &Point, p: &Point) -> bool {
    let v1 = b - a;
    let v2 = p - a;
    v1.x * v2.y > v2.x * v1.y
}

fn jarvis_march<'a>(points: &'a[Point]) -> Vec<&'a Point> {
    let mut hull = Vec::new();

    // Start with the leftmost point
    let mut point_on_hull = points.iter().fold(&points[0], |p1, p2| if p1.x < p2.x { p1 } else { p2 });

    // Keep adding points until we reach the first point again
    loop {
        hull.push(point_on_hull);

        point_on_hull = points.iter().fold(&points[0], |potential, current| {
            if potential == point_on_hull || is_left_of(point_on_hull, potential, current) {
                current
            } else {
                potential
            }
        });

        if point_on_hull == hull[0] { break; } 
    }

    hull
}

fn main() {
    let points = vec![
        Point{ x: 1.0, y: 3.0 },
        Point{ x: 2.0, y: 4.0 },
        Point{ x: 4.0, y: 0.0 },
        Point{ x: 1.0, y: 0.0 },
        Point{ x: 0.0, y: 2.0 },
        Point{ x: 2.0, y: 2.0 },
        Point{ x: 3.0, y: 4.0 },
        Point{ x: 3.0, y: 1.0 }];

    let hull = jarvis_march(&points);
    println!("{:?}", hull);
}
