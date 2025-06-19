//! This is a simple program that loads a list of 3x Lat/Lng pairs from a file
//! It will calculate the total distance between each triplet of points
//! and print some benchmarks to the console

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod distance;

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use crate::distance::{Point, calculate_total_distance};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let path = "../data/points-10000000.json";
    // let path = "../data/points-1000000.json";
    // Load the list of Lat/Lng point triplets from a file
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Point`.
    let points: Vec<Vec<Point>> = match serde_json::from_reader(reader) {
        Ok(points) => points,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            return Err(Box::new(e));
        }
    };
    let elapsed = now.elapsed();
    println!(
        "Loaded {} rows of 3 points from JSON in {:.6?} seconds",
        points.len(),
        elapsed.as_secs_f64()
    );

    // Calculate the distance between each triplet of points
    // and print the total distance
    let now = Instant::now();
    let total_distance = calculate_total_distance(&points).unwrap();
    let elapsed = now.elapsed();
    println!(
        "Calculated total distance of {total_distance:.5} km in {:.6?} seconds",
        elapsed.as_secs_f64()
    );

    Ok(())
}
