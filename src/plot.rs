pub mod plot {

use plotters::prelude::*;

fn point_bounds(points :&Vec<(f64,f64)>) -> (f64,f64,f64,f64){
    let mut first = true; 
    let (mut x_min,mut x_max,mut y_min,mut y_max) = (0.0,0.0,0.0,0.0);
    for (x,y) in points {
        if first {
            (x_min,x_max,y_min,y_max) = (*x,*x,*y,*y)   
        } else {
            if *x<x_min {
                x_min = *x;
            }
            if *x>x_max {
                x_max = *x;
            }
            if *y<y_min {
                y_min = *y;
            }
            if *y>y_max {
                y_max = *y;
            }
        }
        first = false;
    }
    (x_min,x_max,y_min,y_max)
}

const OUT_FILE_NAME: &str = "plotters-doc-data/normal-dist.png";
pub fn plot(points: Vec<(f64,f64)>,convex_hull: Vec<(f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    let aspect_ratio = 768.0/1024.0;
    let (x_min,x_max,y_min,y_max) = point_bounds(&points);
    let x_middle = (x_min + x_max)/2.0;
    let y_middle = (y_min + y_max)/2.0;
    let (x_left,x_right,y_left,y_right) 
        = if (x_max-x_min)*aspect_ratio >= y_max-y_min {
             let height=(x_max-x_min)*aspect_ratio; 
             (x_min,x_max,y_middle-height/2.0,y_middle+height/2.0)
          }else{
             let width=(y_max-y_min)/aspect_ratio;
             (x_middle-width/2.0,x_middle+width/2.0,y_min,y_max)
          };

    root.fill(&WHITE)?;

    let areas = root.split_by_breakpoints([944], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_left..x_right, y_left..y_right)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 10, BLUE.filled())),
    )?;
    scatter_ctx.draw_series(
        convex_hull
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 10, RED.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

}
