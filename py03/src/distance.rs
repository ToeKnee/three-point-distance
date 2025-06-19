//! This module provides functionality to calculate distances between geographical points
//! using the Haversine formula.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// A geographical point represented by latitude and longitude.
#[pyclass(get_all)]
#[derive(Debug, Clone, Default)]
pub struct Point {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
}
#[pymethods]
impl Point {
    /// Create a new Point with the given latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use three_point_distance::distance::Point;
    /// let point = Point::new(45.0, 90.0).unwrap();
    /// assert_eq!(point.latitude, 45.0);
    /// assert_eq!(point.longitude, 90.0);
    /// ```
    ///
    /// # Errors
    ///
    /// If the latitude or longitude is out of bounds, an error is returned.
    #[new]
    pub fn new(latitude: f64, longitude: f64) -> PyResult<Self> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(PyValueError::new_err(
                "Latitude must be between -90 and 90 degrees",
            ));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(PyValueError::new_err(
                "Longitude must be between -180 and 180 degrees",
            ));
        }

        Ok(Self {
            latitude,
            longitude,
        })
    }

    /// Return a Python representation of the Point.
    pub fn __repr__(&self) -> String {
        format!(
            "Point(latitude: {}, longitude:{})",
            self.latitude, self.longitude
        )
    }

    /// Calculate the distance between two points using the Haversine formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use three_point_distance::distance::Point;
    /// let point = Point::new(52.5200, 13.4050).unwrap(); // Berlin
    /// let point2 = Point::new(48.8566, 2.3522).unwrap(); // Paris
    /// const EXPECTED_ERROR: f64 = 0.1;
    /// let expected_distance = 877.463;
    /// let actual_distance = point.distance(&point2);
    /// assert!((actual_distance - expected_distance).abs() < EXPECTED_ERROR);
    /// ```
    pub fn distance(&self, other: &Self) -> f64 {
        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let lon2 = other.longitude.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().mul_add(
            (dlat / 2.0).sin(),
            lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2),
        );
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        6371.0 * c // Radius of Earth in kilometers
    }
}

/// Calculate the distance between three points.
#[pyfunction]
pub fn calculate_distance(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    p1.distance(p2) + p2.distance(p3)
}

/// Calculate the total distance for a list of triplets of points.
#[pyfunction]
pub fn calculate_total_distance<'py>(value: &Bound<'py, PyAny>) -> PyResult<f64> {
    // Convert a Python list of triplets of points into a rust vec of triplets of Points.
    let mut points: Vec<Vec<Point>> = Vec::new();
    match value.downcast::<PyList>() {
        Ok(data) => {
            for item in data.iter() {
                match item.downcast::<PyList>() {
                    Ok(triplet) => {
                        let mut point_triplet = Vec::new();
                        for point in triplet.iter() {
                            match point.extract::<Point>() {
                                Ok(p) => point_triplet.push(p),
                                Err(_) => {
                                    return Err(PyValueError::new_err("Can't convert to Point"));
                                }
                            }
                        }
                        if point_triplet.len() == 3 {
                            points.push(point_triplet);
                        } else {
                            return Err(PyValueError::new_err(
                                "Each triplet must contain exactly 3 points",
                            ));
                        }
                    }
                    Err(_) => {
                        return Err(PyValueError::new_err("Can't convert to Vec<Vec<Point>>"));
                    }
                }
            }
        }
        Err(_) => {
            return Err(PyValueError::new_err(
                "Can't convert to Vec<Bound<'_, PyAny>>",
            ));
        }
    };

    // This code is the same as the standard rust implementation, but we have disable the triple length check as we
    //  already check it in the conversion above.
    let mut total_distance = 0.0;
    for triplet in points {
        // The following line is commented out because we already check the length of the triplet in the conversion above.
        // if triplet.len() != 3 {
        //     return Err(PyValueError::new_err(
        //         "Error: triplet must contain exactly 3 points",
        //     ));
        // }
        let p1 = &triplet[0];
        let p2 = &triplet[1];
        let p3 = &triplet[2];
        total_distance += calculate_distance(p1, p2, p3);
    }
    Ok(total_distance)
}

/// Calculate the totals distance for a list of triplets of points from a Ruby array.
/// This is a convenience function to allow the user to pass a Ruby array of triplets of points.
///
/// # Errors
/// If the array is empty or if any triplet does not contain exactly 3 points, an error is returned.
///
/// # Panics
/// If the triplet can not be unwraped.
#[pyfunction]
pub fn calculate_total_distance_from_array<'py>(
    array_of_points: &Bound<'py, PyAny>,
) -> PyResult<f64> {
    let array_of_points = match array_of_points.downcast::<PyList>() {
        Ok(list) => list,
        Err(_) => {
            return Err(PyValueError::new_err(
                "Expected a list of triplets of points",
            ));
        }
    };
    let points = array_of_points
        .iter()
        .map(|item| match item.downcast::<PyList>() {
            Ok(triplet) => {
                let mut point_triplet = Vec::new();
                for point in triplet.iter() {
                    let lat: f64 = match point.get_item(0) {
                        Ok(lat) => lat.extract().unwrap_or_default(),
                        Err(_) => return Err(PyValueError::new_err("Can't extract latitude")),
                    };
                    let lng: f64 = match point.get_item(1) {
                        Ok(lng) => lng.extract().unwrap_or_default(),
                        Err(_) => return Err(PyValueError::new_err("Can't extract longitude")),
                    };
                    point_triplet.push(Point::new(lat, lng).unwrap_or_else(|e| {
                        eprintln!("Error: {e}");
                        Point::default()
                    }));
                }
                if point_triplet.len() == 3 {
                    Ok(point_triplet)
                } else {
                    Err(PyValueError::new_err(
                        "Each triplet must contain exactly 3 points",
                    ))
                }
            }
            Err(_) => Err(PyValueError::new_err("Can't convert to Vec<Vec<Point>>")),
        })
        .collect::<Result<Vec<Vec<Point>>, PyErr>>()?;
    //    println!("array_of_points: {:?}", array_of_points);
    if points.is_empty() {
        return Ok(0.0);
    }

    let mut total_distance = 0.0;
    for mut triplet in points {
        let [p1, p2, p3] = triplet.as_mut_slice() else {
            return Err(PyValueError::new_err(
                "Each triplet must contain exactly 3 points",
            ));
        };

        total_distance += calculate_distance(p1, p2, p3);
    }
    Ok(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_equalish(x: f64, y: f64) {
        const EXPECTED_ERROR: f64 = 0.1;
        assert!((x - y).abs() < EXPECTED_ERROR);
    }

    #[test]
    fn test_latitude() {
        assert_equalish(Point::new(45.0, 90.0).unwrap().latitude, 45.0);
        assert_equalish(Point::new(-90.0, 90.0).unwrap().latitude, -90.0);
        assert!(Point::new(91.0, 90.0).is_err());
    }

    #[test]
    fn test_longitude() {
        assert_equalish(Point::new(45.0, 45.0).unwrap().longitude, 45.0);
        assert_equalish(Point::new(45.0, -180.0).unwrap().longitude, -180.0);
        assert!(Point::new(45.0, -181.0).is_err());
    }

    #[test]
    fn test_point() {
        let point = Point::new(45.0, 90.0).unwrap();
        assert_equalish(point.latitude, 45.0);
        assert_equalish(point.longitude, 90.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(52.5200, 13.4050).unwrap(); // Berlin
        let p2 = Point::new(48.8566, 2.3522).unwrap(); // Paris
        let distance = p1.distance(&p2);
        assert_equalish(distance, 877.463);
    }

    #[test]
    fn test_calculate_distance() {
        let p1 = Point::new(45.0, 90.0).unwrap();
        let p2 = Point::new(46.0, 91.0).unwrap();
        let p3 = Point::new(47.0, 92.0).unwrap();
        let distance = calculate_distance(&p1, &p2, &p3);
        assert_equalish(distance, 270.7754);
    }
}
