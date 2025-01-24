pub mod graham_scan{

use partial_min_max::{min,max};
use std::cmp::Ordering::{Equal,Less,Greater};


#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
enum VSide {
    Top,
    Bottem
}


fn is_vside(vside : VSide, (x1,y1) : (f64,f64), (x2,y2) : (f64,f64), (x3,y3) : (f64,f64)) -> bool {
    let y = if x1==x3 {if vside==VSide::Top {max(y1,y3)} else {min(y1,y3)}} else {(y3-y1)/(x3-x1)*(x2-x1)+y1};
    if vside == VSide::Top {
        return y>=y2;
    }else{
        return y<=y2;
    }
}


fn convex_hull_vside (vside : VSide, sorted_points : &[(f64,f64)]) -> Vec<(f64,f64)> {
    let mut convex_hull: Vec<(f64,f64)> = Vec::new();
    for point in sorted_points {
        if convex_hull.len()<2{
            convex_hull.push(*point);
            continue
        }
        let mut i = convex_hull.len()-1;
        while i>0 {
            if is_vside(vside,convex_hull[i-1],convex_hull[i],*point) {
                convex_hull.pop();
            }else{
                break;
            }
            i-=1;
        }
        convex_hull.push(*point);
    }
    return convex_hull;
}

//This function assumes that a and b have the same ordering.
fn combine_ccw<T: PartialOrd+Copy>(top:Vec<T>,bottem:Vec<T>) -> Vec<T>{
    let mut vec = bottem.clone();
    let mut i = top.len();
    let mut j = bottem.len();
    loop {
        match (i>0,j>0) {
            (true,true) => match top[i-1].partial_cmp(&bottem[j-1]) {
                                Some(Equal)   => i-=1,
                                Some(Less)    => j-=1,
                                Some(Greater) => {vec.push(top[i-1]);i-=1;}
                                None          => todo!()
                           }
            (true,false)=> {vec.push(top[i-1]);i-=1;}
            (false,_)   => break
        }
    }
    return vec;
}



pub fn graham_scan(mut points : Vec<(f64,f64)>) -> Vec<(f64,f64)> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let convex_hull_top= convex_hull_vside(VSide::Top, &points);
    let convex_hull_bottem = convex_hull_vside(VSide::Bottem, &points);
    let convex_hull = combine_ccw(convex_hull_top,convex_hull_bottem);
    //println!("All the points {:?}",points);
    //println!("convex_hull_top: {:?}",convex_hull_top);
    //println!("convex_hull_bottem: {:?}",convex_hull_bottem);
    return convex_hull ;
    //println!("convex_hull: {:?}",convex_hull);
}
}
