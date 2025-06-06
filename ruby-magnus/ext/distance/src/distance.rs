//! This module provides functionality to calculate distances between geographical points
//! using the Haversine formula.
//!
//! This is based on the original code from the `three-point-distance` crate. But has been adapted for magnus.

use magnus::{
    scan_args::{get_kwargs, scan_args},
    typed_data, Error, RArray, Ruby, Value,
};
use std::cell::RefCell;

/// A geographical point represented by latitude and longitude.
#[magnus::wrap(class = "Point", free_immediately, size)]
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Point {
    /// Latitude in degrees.
    pub latitude: RefCell<f64>,
    /// Longitude in degrees.
    pub longitude: RefCell<f64>,
}
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
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, String> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Latitude must be between -90 and 90 degrees".to_string());
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Longitude must be between -180 and 180 degrees".to_string());
        }

        let point = Self {
            latitude: RefCell::new(latitude),
            longitude: RefCell::new(longitude),
        };
        Ok(point)
    }

    /// Initialize a Point from Ruby arguments.
    ///
    /// # Errors
    /// If the latitude or longitude is out of bounds, an error is returned.
    pub fn initialize(
        ruby: &Ruby,
        rb_self: typed_data::Obj<Self>,
        args: &[Value],
    ) -> Result<(), Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let args = get_kwargs(args.keywords, &["latitude", "longitude"], &[])?;
        //let (latitude, longitude): (f64, f64) = args.required;
        let (latitude, longitude): (f64, f64) = args.required;
        let (): () = args.optional;
        let _: () = args.splat;

        if !(-90.0..=90.0).contains(&latitude) {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "Latitude must be between -90 and 90 degrees",
            ));
        }
        *rb_self.latitude.borrow_mut() = latitude;

        if !(-180.0..=180.0).contains(&longitude) {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "Longitude must be between -180 and 180 degrees",
            ));
        }
        *rb_self.longitude.borrow_mut() = longitude;

        Ok(())
    }

    /// Get the latitude of the point.
    pub fn latitude(&self) -> f64 {
        *self.latitude.borrow()
    }

    /// Get the latitude of the point.
    pub fn longitude(&self) -> f64 {
        *self.longitude.borrow()
    }

    /// Display the point as a string.
    pub fn to_s(&self) -> String {
        format!(
            "Point.new(latitude: {}, longitude: {})",
            self.latitude(),
            self.longitude()
        )
    }

    /// Calculate the distance between two points using the Haversine formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::distance::Point;
    /// let point = Point::new(52.5200, 13.4050).unwrap(); // Berlin
    /// let point2 = Point::new(48.8566, 2.3522).unwrap(); // Paris
    /// assert_eq!(point.distance(&point2), 877.463);
    /// ```
    #[must_use]
    pub fn distance(&self, other: &Self) -> f64 {
        let lat1 = self.latitude.borrow().to_radians();
        let lon1 = self.longitude.borrow().to_radians();
        let lat2 = other.latitude.borrow().to_radians();
        let lon2 = other.longitude.borrow().to_radians();

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
pub fn calculate_distance(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    p1.distance(p2) + p2.distance(p3)
}

/// Calculate the total distance for a list of triplets of points.
///
/// # Errors
/// If the array is empty or if any triplet does not contain exactly 3 points, an error is returned.
///
/// # Panics
/// If the triplet can not be unwrapped.
#[must_use]
pub fn calculate_total_distance(points: RArray) -> f64 {
    let mut total_distance = 0.0;
    for triplet in points {
        let triplet = RArray::from_value(triplet).unwrap();
        if triplet.len() != 3 {
            eprintln!("Error: triplet must contain exactly 3 points");
            return -1.0;
        }
        let p1 = triplet.pop().unwrap();
        let p2 = triplet.pop().unwrap();
        let p3 = triplet.pop().unwrap();
        total_distance += calculate_distance(p1, p2, p3);
    }
    total_distance
}

/// Calculate the totals distance for a list of triplets of points from a Ruby array.
/// This is a convenience function to allow the user to pass a Ruby array of triplets of points.
///
/// # Errors
/// If the array is empty or if any triplet does not contain exactly 3 points, an error is returned.
///
/// # Panics
/// If the triplet can not be unwraped.
pub fn calculate_total_distance_from_array(
    ruby: &Ruby,
    array_of_points: RArray,
) -> Result<f64, Error> {
    if array_of_points.is_empty() {
        return Ok(0.0);
    }

    let mut points: Vec<_> = vec![];
    for triplet in array_of_points {
        let triplet = RArray::from_value(triplet).unwrap();
        if triplet.len() != 3 {
            eprintln!("Error: triplet must contain exactly 3 points");
            return Err(Error::new(
                ruby.exception_arg_error(),
                "Triplet must contain exactly 3 points",
            ));
        }

        let mut point_triplet = vec![];
        for p in triplet {
            let p = RArray::from_value(p).unwrap();
            let longitude: f64 = p.pop().unwrap();
            let latitude: f64 = p.pop().unwrap();

            let p = Point::new(latitude, longitude).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                Point::default()
            });
            point_triplet.push(p);
        }
        points.push(point_triplet);
    }

    let mut total_distance = 0.0;
    for mut triplet in points {
        let [p1, p2, p3] = triplet.as_mut_slice() else {
            return Err(Error::new(
                ruby.exception_arg_error(),
                "Triplet must contain exactly 3 points",
            ));
        };

        total_distance += calculate_distance(p1, p2, p3);
    }
    Ok(total_distance)
}
