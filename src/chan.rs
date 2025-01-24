use std::cmp::{min};
use std::cmp::Ordering::Equal;
//use partial_min_max::max;
//mod graham_scan;
use crate::graham_scan::graham_scan;

fn norm ((x,y): (f64,f64)) -> f64 {
    f64::sqrt(x*x+y*y)
}

fn cos_angle((x1,y1): (f64,f64), (x2,y2): (f64,f64), (x3,y3): (f64,f64))->f64 {
    ((x1-x2)*(x3-x2)+(y1-y2)*(y3-y2))/norm((x1-x2,y1-y2))/norm((x3-x2,y3-y2))
}

fn chan_m(points : Vec<(f64,f64)>, m : i64) -> Option<Vec<(f64,f64)>> {
    //partition P into subsets P1 . . . . . Prn/m] each of size at most m
    let mut i =0;
    let mut partition = Vec::new();
    for point in &points {
        if i%m==0 {
            partition.push(Vec::new());
        }
        partition[(i/m) as usize ].push(*point);
        i+=1;
    }
    let mut partition_convex_hulls = Vec::new();
    for subset in partition {
        partition_convex_hulls.push(graham_scan(subset));
    }
    let mut convex_hull = Vec::new();
    let Some(rightmost) = points.iter().max_by(|a,b| a.partial_cmp(b).unwrap_or(Equal)) else {todo!()};
    convex_hull.push(*rightmost);
    for k in 1..m {
        let mut min_cos_angle = f64::INFINITY;
        let mut new_p: Option<(f64,f64)> = None;
        for partition_convex_hull in &partition_convex_hulls {
            let prev_p = 
            if k==1 {
                let (x,y)=convex_hull[0];(x,y-1.0)
            } else {
                convex_hull[(k-2) as usize]
            };
            let p = convex_hull[(k-1) as usize];
            let cos_angle_to = |q| cos_angle(prev_p,p,q);
            let Some(q_i) = partition_convex_hull.iter().filter(|a| **a!=p).min_by(|a,b| cos_angle_to(**a).partial_cmp(&cos_angle_to(**b)).unwrap_or(Equal)) else {continue;};
            let q_i_cos_angle_to = cos_angle_to(*q_i);
            if q_i_cos_angle_to< min_cos_angle {
                min_cos_angle = q_i_cos_angle_to;
                new_p = Some(*q_i);
            }
        }
        if new_p==Some(convex_hull[0]) {
            return Some(convex_hull) ;
        } else{
            convex_hull.push(new_p?);
        }
    }
    None
}

pub fn chan(points : Vec<(f64,f64)>) -> Vec<(f64,f64)> {
    let mut t=1;
    loop{
        let m = min(2^(2^t),points.len());
        match chan_m(points.clone(),m as i64) {
            Some(hull) => return hull,
            None => {}
        }
        t+=1;
    }
}
