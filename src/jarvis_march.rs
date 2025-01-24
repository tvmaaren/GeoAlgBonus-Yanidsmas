// Jarvis March algorithm (or gift wrapping algorithm) to find the convex hull given a set of points
// The algorithm uses the cross product to calculate the point most 'outer' point in clockwise direction, starting at the most left point, which will definitely be on the convex hull.

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    // Case we only have 1 (or 0) points
    if points.len() <= 1 {
        return points;
    }

    // Start the convex hull at the leftmost point -> this point will always be on the convex hull
    let leftmost = points.iter().min_by_key(|p| p.x).unwrap();

    let mut convex_hull_result = Vec::new();
    let mut current_point = *leftmost;

    loop {
        convex_hull_result.push(current_point);
        let mut next_point = points[0];

        for &candidate in &points {
            if candidate == current_point {
                continue;
            }

            let cross_product_result = cross_product(current_point, next_point, candidate);
            if cross_product_result > 0 || (cross_product_result == 0 && distance_squared(current_point, candidate) > distance_squared(current_point, next_point)) {
                next_point = candidate;
            }
        }

        current_point = next_point;
        if current_point == *leftmost {
            break;
        }
    }

    convex_hull_result
}

fn cross_product(p1: Point, p2: Point, p3: Point) -> i32 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

fn distance_squared(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)
}

fn main() {
    let set_of_points = vec![
        // The set of points can be defined here like this point:
        Point::new(4, 5),
    ];

    let convex_hull_result = convex_hull(set_of_points);

    println!("These points make the convex hull:");
    for point in convex_hull_result {
        println!("({}, {})", point.x, point.y);
    }
}
