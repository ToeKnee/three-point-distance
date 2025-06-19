//! This is a simple program that loads a list of 3x Lat/Lng pairs from a file
//! It will calculate the total distance between each triplet of points
//! and print some benchmarks to the console
//!
//! This is a WebAssembly module that can be used in a web application
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod distance;

use crate::distance::{Point, calculate_total_distance};
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_time::Instant;

async fn load_json() -> Result<Vec<Vec<Point>>, JsValue> {
    // let url = "http://127.0.0.1:8000/points-10000000.json"; // Full dataset used in the rest of the examples, too big for the browser
    let url = "http://127.0.0.1:8000/points-1000000.json"; // Same as regular JS
    // let url = "http://127.0.0.1:8000/points-10.json"; // Quick test
    let response = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            console::log_1(&format!("Error fetching JSON: {}", err).into());
            return Err(JsValue::from_str(&format!("Error fetching JSON: {}", err)));
        }
    };

    let body = match response.text().await {
        Ok(text) => text,
        Err(err) => {
            console::log_1(&format!("Error reading response text: {}", err).into());
            return Err(JsValue::from_str(&format!(
                "Error reading response text: {}",
                err
            )));
        }
    };

    // Read the JSON contents of the file as an instance of `Point`.
    let points: Vec<Vec<Point>> = match serde_json::from_str(&body) {
        Ok(points) => points,
        Err(e) => {
            return Err(JsValue::from_str(&format!("Error reading file: {e}")));
        }
    };

    Ok(points)
}

/// This is equivelent to the `main` function in the Rust binary. It loads the JSON from a URL instead of from disk.
#[wasm_bindgen]
pub async fn main() -> Result<f64, JsValue> {
    // This is where you can call your Rust functions or perform any initialization
    let now = Instant::now();
    let mut points = load_json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Error loading JSON: {:?}", e)))?;
    let elapsed = now.elapsed();
    console::log_1(
        &format!(
            "Loaded {} rows of 3 points from JSON in {:.6?} seconds",
            points.len(),
            elapsed.as_secs_f64()
        )
        .into(),
    );

    // Calculate the distance between each triplet of points
    // and print the total distance
    let now = Instant::now();
    let mut total_distance = calculate_total_distance(&points).unwrap();
    let elapsed = now.elapsed();
    console::log_1(
        &format!(
            "Calculated total distance of {total_distance:.5} km in {:.6?} seconds",
            elapsed.as_secs_f64()
        )
        .into(),
    );

    console::log_1(& "Run the load and calculate 10 times to simulate the amount of data processes in the non-browser versions.".into());

    let now = Instant::now();

    for _ in 0..10 {
        points = load_json()
            .await
            .map_err(|e| JsValue::from_str(&format!("Error loading JSON: {:?}", e)))?;
    }

    let elapsed = now.elapsed();
    console::log_1(
        &format!(
            "Loaded {} rows of 3 points from JSON in {:.6?} seconds - 10 times",
            points.len(),
            elapsed.as_secs_f64()
        )
        .into(),
    );

    let now = Instant::now();
    for _ in 0..10 {
        total_distance = calculate_total_distance(&points).unwrap();
    }
    let elapsed = now.elapsed();
    console::log_1(
        &format!(
            "Calculated total distance of {total_distance:.5} km in {:.6?} seconds",
            elapsed.as_secs_f64()
        )
        .into(),
    );

    Ok(total_distance)
}
