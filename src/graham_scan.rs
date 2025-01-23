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

//This function assumes that a and b are in the same order.
fn union_sorted<T: PartialOrd+Copy>(a:Vec<T>,b:Vec<T>) -> Vec<T>{
    let mut vec = Vec::new();
    let mut i = 0;
    let mut j = 0;
    loop {
        match (i<a.len(),j<b.len()) {
            (true,true) => match a[i].partial_cmp(&b[j]) {
                                Some(Equal)   => {vec.push(a[i]);
                                                  i+=1;
                                                  j+=1
                                                 }
                                Some(Less)    => {vec.push(a[i]);
                                                  i+=1;
                                                 }
                                Some(Greater) => {vec.push(b[j]);
                                                  j+=1;
                                                 }
                                None          => todo!()
                           }
            (true,false)=> {vec.push(a[i]);
                           i+=1;}
            (false,true)=> {vec.push(b[i]);
                           j+=1;}
            (false,false)=>break,
        }
    }
    vec.push(a[0]);
    return vec;
}



pub fn graham_scan(mut points : Vec<(f64,f64)>) -> Vec<(f64,f64)> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let convex_hull_top= convex_hull_vside(VSide::Top, &points);
    let convex_hull_bottem = convex_hull_vside(VSide::Bottem, &points);
    let convex_hull = union_sorted(convex_hull_top,convex_hull_bottem);
    //println!("All the points {:?}",points);
    //println!("convex_hull_top: {:?}",convex_hull_top);
    //println!("convex_hull_bottem: {:?}",convex_hull_bottem);
    return convex_hull ;
    //println!("convex_hull: {:?}",convex_hull);
}
}
