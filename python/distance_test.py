import unittest

from distance import Point, calculate_distance, calculate_total_distance


class TestPoint(unittest.TestCase):
    def test_point_initialization(self):
        # Test valid initialization
        p = Point(45.0, 90.0)
        assert p.x == 45.0
        assert p.y == 90.0

        # Test invalid latitude
        try:
            Point(100.0, 90.0)
        except ValueError as e:
            assert str(e) == "Latitude 100.0 is out of bounds"

        # Test invalid longitude
        try:
            Point(45.0, 200.0)
        except ValueError as e:
            assert str(e) == "Longitude 200.0 is out of bounds"

    def test_distance(self):
        point1 = Point(52.5200, 13.4050) # Berlin
        point2 = Point(48.8566, 2.3522)  # Paris

        # Test distance between two points
        assert abs(point1.distance(point2) - 877.463) < 1e-3

class TestDistanceFunctions(unittest.TestCase):
    def test_calculate_distance(self):
        point1 = Point(45.0, 90.0)
        point2 = Point(46.0, 91.0)
        point3 = Point(47.0, 92.0)

        # Test distance calculation for a triplet of points
        assert abs(calculate_distance(point1, point2, point3) - 270.7754) < 1e-3

    def test_calculate_total_distance(self):
        points = [
            [Point(45.0, 90.0), Point(46.0, 91.0), Point(47.0, 92.0)],
            [Point(48.0, 93.0), Point(49.0, 94.0), Point(50.0, 95.0)]
        ]

        # Test total distance calculation
        assert abs(calculate_total_distance(points) - (536.74756)) < 1e-3

if __name__ == "__main__":
    unittest.main()
