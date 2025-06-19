//! This library provides functionality to calculate distances between geographical points
//! It exposes a Rust implemtation of the Haversine formula to Ruby

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod distance;

use crate::distance::{
    calculate_distance, calculate_total_distance, calculate_total_distance_from_array, Point,
};
use magnus::{function, method, prelude::*, Error, Ruby};

/// This function initializes the Ruby extension by defining the `Point` class, and `calculate_distance`, `calculate_total_distance`, and `calculate_total_distance_from_array` methods.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Point", ruby.class_object())?;
    // Define alloc func based on the Default impl, plus an initialize method,
    // rather than overwriting `new`, to allow class to be subclassed from Ruby
    class.define_alloc_func::<Point>();
    class.define_method("initialize", method!(Point::initialize, -1))?;
    class.define_method("to_s", method!(Point::to_s, 0))?;
    class.define_method("latitude", method!(Point::latitude, 0))?;
    class.define_method("longitude", method!(Point::longitude, 0))?;
    class.define_method("distance", method!(Point::distance, 1))?;

    ruby.define_global_function("calculate_distance", function!(calculate_distance, 3));
    ruby.define_global_function(
        "calculate_total_distance",
        function!(calculate_total_distance, 1),
    );
    ruby.define_global_function(
        "calculate_total_distance_from_array",
        function!(calculate_total_distance_from_array, 1),
    );
    Ok(())
}
