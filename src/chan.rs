use std::cmp::min;
use partial_min_max::max;
//mod graham_scan;
use crate::graham_scan::graham_scan::graham_scan;

fn chan_m(points : Vec<(f64,f64)>, m : i64) -> Option<Vec<(f64,f64)>> {
    //partition P into subsets P1 . . . . . Prn/m] each of size at most m
    let mut i =0;
    let mut partition = Vec::new();
    for point in points {
        if i%m==0 {
            partition.push(Vec::new());
        }
        partition[(i/m) as usize ].push(point);
        i+=1;
    }
    let mut partition_convex_hulls = Vec::new();
    for subset in partition {
        partition_convex_hulls.push(graham_scan(subset));
    }
    let mut convex_hull = Vec::new();
    convex_hull.push(max(points));
    for k in [1..m] {
        for partition_convex_hull in partition_convex_hulls {
            if(k
        }
    }
    todo!()
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
