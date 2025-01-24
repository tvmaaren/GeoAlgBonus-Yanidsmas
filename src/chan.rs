use std::cmp::{min};
use std::cmp::Ordering::Equal;
//use partial_min_max::max;
//mod graham_scan;
use crate::graham_scan::graham_scan;

fn norm ((x,y): (f64,f64)) -> f64 {
    f64::sqrt(x*x+y*y)
}

fn dist((x1,y1): (f64,f64), (x2,y2): (f64,f64)) -> f64 {
    norm((x1-x2,y1-y2))
}

fn cos_angle((x1,y1): (f64,f64), (x2,y2): (f64,f64), (x3,y3): (f64,f64))->f64 {
    ((x1-x2)*(x3-x2)+(y1-y2)*(y3-y2))/norm((x1-x2,y1-y2))/norm((x3-x2,y3-y2))
}

#[derive(PartialEq, Copy, Clone)]
enum Change {
    Increase,
    Decrease,
}

fn min_bs(convex_vertices: &Vec<(f64,f64)>, p: (f64,f64), prev_p: (f64,f64)) -> Option<(f64,f64)> {
    if convex_vertices.len() == 0 {
        return None ;
    }
    if convex_vertices.len() == 1 {
        if convex_vertices[0] == p {
            return None ;
        }
        return Some(convex_vertices[0]);
    }
    
    let cos_angle_dist = |q| (cos_angle(prev_p,p,q),-dist(q,p));
    let (mut l_change,mut r_change) = 
    if cos_angle_dist(convex_vertices[0])<cos_angle_dist(convex_vertices[1]) {
        (Change::Increase,Change::Increase)
    } else {
        (Change::Decrease,Change::Decrease)
    };
    let mut l = 0;
    let n = convex_vertices.len();
    let mut r = n;
    while l+1<r{
        let m = (l+r)/2;
        let m_change = if cos_angle_dist(convex_vertices[m])<cos_angle_dist(convex_vertices[(m+1)%n]) {
                            Change::Increase
                        }else {
                            Change::Decrease
                        };
        let left = (l,m,l_change,m_change);
        let right =(m,r,m_change,r_change);
        (l,r,l_change,r_change) = 
        match (l_change,m_change,r_change,cos_angle_dist(convex_vertices[l])<cos_angle_dist(convex_vertices[m])) {
            (Change::Decrease,Change::Decrease,Change::Decrease, true) =>left,
            (Change::Decrease,Change::Decrease,Change::Decrease,false) =>right,
            (Change::Increase,               _,Change::Decrease,    _) =>todo!("This should not happen"),
            (               _,Change::Decrease,Change::Increase,    _) =>right,
            (Change::Decrease,Change::Increase,               _,    _) =>left,
            (Change::Increase,Change::Increase,Change::Increase, true) =>right,
            (Change::Increase,Change::Increase,Change::Increase,false) =>left,
            
        };
    }
    let i = if cos_angle_dist(convex_vertices[l])<cos_angle_dist(convex_vertices[(l+1)%n]) {l} else {(l+1)%n};
    if convex_vertices[i]==p {Some(convex_vertices[(i+1)%n])} else {Some(convex_vertices[i])}
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
        let mut min_cos_angle_dist = (f64::INFINITY,f64::INFINITY);
        let mut new_p: Option<(f64,f64)> = None;
        for partition_convex_hull in &partition_convex_hulls {
            let prev_p = 
            if k==1 {
                let (x,y)=convex_hull[0];(x,y-1.0)
            } else {
                convex_hull[(k-2) as usize]
            };
            let p = convex_hull[(k-1) as usize];
            let cos_angle_dist = |q| (cos_angle(prev_p,p,q),-dist(q,p));
            //let Some(q_i) = partition_convex_hull.iter().filter(|a| **a!=p).min_by(|a,b| (cos_angle_dist(**a).partial_cmp(&cos_angle_dist(**b)).unwrap_or(Equal))) else {continue;};
            match min_bs(partition_convex_hull,p,prev_p) {
                Some(q_i) =>{
                            let q_i_cos_angle_dist = cos_angle_dist(q_i);
                            if q_i_cos_angle_dist< min_cos_angle_dist {
                                min_cos_angle_dist = q_i_cos_angle_dist;
                                new_p = Some(q_i);
                            }
                            }
                _         =>{}
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
        let m = min(2_i64.pow(2_u32.pow(t)),points.len() as i64);
        println!("{} {} {}",m,2^(2^t),points.len());
        match chan_m(points.clone(),m as i64) {
            Some(hull) => return hull,
            None => {}
        }
        t+=1;
    }
}
