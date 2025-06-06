//! This module provides functionality to calculate distances between geographical points
//! using the Haversine formula.
use serde::{Deserialize, Serialize};

/// A geographical point represented by latitude and longitude.
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
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

        Ok(Self {
            latitude,
            longitude,
        })
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
    #[must_use]
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
#[must_use]
pub fn calculate_distance(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    p1.distance(p2) + p2.distance(p3)
}

/// Calculate the total distance for a list of triplets of points.
#[must_use]
pub fn calculate_total_distance(points: &[Vec<Point>]) -> f64 {
    let mut total_distance = 0.0;
    for triplet in points {
        if triplet.len() != 3 {
            eprintln!("Error: triplet must contain exactly 3 points");
            return -1.0;
        }
        let p1 = &triplet[0];
        let p2 = &triplet[1];
        let p3 = &triplet[2];
        total_distance += calculate_distance(p1, p2, p3);
    }
    total_distance
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

    #[test]
    fn test_total_distance() {
        let p1 = Point::new(45.0, 90.0).unwrap();
        let p2 = Point::new(46.0, 91.0).unwrap();
        let p3 = Point::new(47.0, 92.0).unwrap();
        let p4 = Point::new(48.0, 93.0).unwrap();
        let p5 = Point::new(49.0, 94.0).unwrap();
        let p6 = Point::new(50.0, 95.0).unwrap();
        let points = vec![vec![p1, p2, p3], vec![p4, p5, p6]];
        let total_distance = calculate_total_distance(&points);
        assert_equalish(total_distance, 536.74756);
    }
}
