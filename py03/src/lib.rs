pub mod distance;

use distance::{
    Point, calculate_distance, calculate_total_distance, calculate_total_distance_from_array,
};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn fast_distance(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_function(wrap_pyfunction!(calculate_distance, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_total_distance, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_total_distance_from_array, m)?)?;

    Ok(())
}
