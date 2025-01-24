// Jarvis March algorithm (or gift wrapping algorithm) to find the convex hull given a set of points
// The algorithm uses the cross product to calculate the point most 'outer' point in clockwise direction, starting at the most left point, which will definitely be on the convex hull.

use std::cmp::Ordering::Equal;


pub fn jarvis_march(points: Vec<(f64,f64)>) -> Vec<(f64,f64)> {
    // Case we only have 1 (or 0) points
    if points.len() <= 1 {
        return points;
    }

    // Start the convex hull at the leftmost point -> this point will always be on the convex hull
    let leftmost = points.iter().min_by(|a,b| a.partial_cmp(b).unwrap_or(Equal)).unwrap();

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
            if cross_product_result > 0.0 || (cross_product_result == 0.0 && distance_squared(current_point, candidate) > distance_squared(current_point, next_point)) {
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

fn cross_product((p1x,p1y): (f64,f64), (p2x,p2y): (f64,f64), (p3x,p3y): (f64,f64)) -> f64 {
    (p2x - p1x) * (p3y - p1y) - (p2y - p1y) * (p3x - p1x)
}

fn distance_squared((p1x,p1y): (f64,f64), (p2x,p2y): (f64,f64)) -> f64 {
    (p1x - p2x).powf(2.0) + (p1y - p2y).powf(2.0)
}
