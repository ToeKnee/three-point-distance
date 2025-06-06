//! This library provides functionality to calculate distances between geographical points

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod distance;

// Re-export the Point struct and calculate_total_distance function
pub use crate::distance::{Point, calculate_total_distance};
