use std::cmp::Ordering::Equal;

// Takes a vector and an index and returns the previous value of the index in the vector
fn previous_index(len: usize, i: usize) -> usize {
    let l: usize = len;
    if i == 0 {
        return l - 1;
    }
    i - 1
}

// Same but for the next one
fn next_index(len: usize, i: usize) -> usize {
    (i + 1) % len
}

// Used for checking if a point is right of a line
fn cross_product(a: (f64, f64), b: (f64, f64), point: (f64, f64)) -> f64 {
    (b.0 - a.0) * (point.1 - a.1) - (b.1 - a.1) * (point.0 - a.0)
}
fn distance_sq(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)
}

// Takes two vectors that are clockwise sorted convex hulls and returns the indices of the points that define their left tangent line.
fn find_left_tangent(a: &[(f64, f64)], b: &[(f64, f64)]) -> (usize, usize) {
    let mut current_a = 0;
    let mut current_b = 0;
    let len_a = a.len();
    let len_b = b.len();
    loop {
        let next_a = next_index(len_a, current_a);
        let previous_a = previous_index(len_a, current_a);

        let cross_next_a = cross_product(a[current_a], b[current_b], a[next_a]);
        let cross_previous_a = cross_product(a[current_a], b[current_b], a[previous_a]);

        // Handle colinearity for a
        if cross_previous_a.abs() < f64::EPSILON
            && distance_sq(a[current_a], b[current_b]) < distance_sq(a[previous_a], b[current_b])
        {
            current_a = previous_a;
            continue;
        }

        // Handle non-colinear points for a
        let next_a_wrong = cross_next_a > 0.0;
        let previous_a_wrong = cross_previous_a > 0.0;
        if next_a_wrong || previous_a_wrong {
            current_a = previous_a;
            continue;
        }

        let next_b = next_index(len_b, current_b);
        let previous_b = previous_index(len_b, current_b);

        let cross_next_b = cross_product(a[current_a], b[current_b], b[next_b]);
        let cross_previous_b = cross_product(a[current_a], b[current_b], b[previous_b]);

        // Handle colinearity for b
        if cross_next_b.abs() < f64::EPSILON
            && distance_sq(a[current_a], b[current_b]) < distance_sq(a[current_a], b[next_b])
        {
            current_b = next_b;
            continue;
        }

        // Handle non-colinear points for b
        let next_b_wrong = cross_next_b > 0.0;
        let previous_b_wrong = cross_previous_b > 0.0;

        if next_b_wrong || previous_b_wrong {
            current_b = next_b;
            continue;
        }
        break;
    }
    (current_a, current_b)
}

// Takes two vectors that are clockwise sorted convex hulls and returns the merged clockwise sorted convex hull.
fn conquer(a: &[(f64, f64)], b: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let (a1, b1) = find_left_tangent(a, b);
    let (b2, a2) = find_left_tangent(b, a);
    let mut merged_hull = Vec::with_capacity(a.len() + b.len());
    let mut avalue = a2;
    loop {
        merged_hull.push(a[avalue]);
        if avalue == a1 {
            break;
        }
        avalue = next_index(a.len(), avalue);
    }
    let mut bvalue = b1;
    loop {
        merged_hull.push(b[bvalue]);
        if bvalue == b2 {
            break;
        }
        bvalue = next_index(b.len(), bvalue);
    }
    merged_hull
}
fn divide(sorted_points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let n = sorted_points.len();
    if n == 1 {
        return vec![sorted_points[0]];
    }
    let mid = n / 2;
    let left_hull = divide(&sorted_points[..mid]);
    let right_hull = divide(&sorted_points[mid..]);
    conquer(&left_hull, &right_hull)
}

pub fn divide_and_conquer(mut points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    divide(&points)
}
