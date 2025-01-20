mod graham_scan;
use crate::graham_scan::graham_scan::graham_scan;
mod plot;
use crate::plot::plot::plot;

use std::fs::File;
use std::io::{self, BufRead};

fn parse_float_tuples_from_file(file_path: &str) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    // Open the file for reading
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors
        // Parse the line as a tuple
        if let Some((first, second)) = parse_tuple(&line) {
            result.push((first, second));
        } else {
            return Err(format!("Failed to parse line: {}", line).into());
        }
    }

    Ok(result)
}

fn parse_tuple(line: &str) -> Option<(f64, f64)> {
    // Remove parentheses and split by comma
    let trimmed = line.trim().trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = trimmed.split(',').map(str::trim).collect();

    if parts.len() == 2 {
        if let (Ok(first), Ok(second)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            return Some((first, second));
        }
    }

    None
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let file_path = "tests/test2";
  let points = parse_float_tuples_from_file(file_path)?;
  //TODO: remove clone
  let convex_hull = graham_scan(points.clone());
  //TODO: Handle errors
  plot(points.clone(),convex_hull)?;
  Ok(())
}
