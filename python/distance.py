from math import radians, sin, cos, sqrt, atan2

class Point:
    def __init__(self, x, y):
        if not (-90 <= x <= 90):
            raise ValueError(f"Latitude {x} is out of bounds")
        if not (-180 <= y <= 180):
            raise ValueError(f"Longitude {y} is out of bounds")
        self.x = x
        self.y = y

    def distance(self, other):
        """Calculate the distance between two points using Haversine formula"""

        R = 6371.0  # Radius of the Earth in kilometers
        lat1, lon1 = radians(self.x), radians(self.y)
        lat2, lon2 = radians(other.x), radians(other.y)
        dlat = lat2 - lat1
        dlon = lon2 - lon1
        a = sin(dlat / 2)**2 + cos(lat1) * cos(lat2) * sin(dlon / 2)**2
        c = 2 * atan2(sqrt(a), sqrt(1 - a))
        return R * c

def calculate_distance(point1, point2, point3):
    return point1.distance(point2) + point2.distance(point3)

def calculate_total_distance(points):
    total_distance = 0.0
    for triplet in points:
        if len(triplet) != 3:
            raise ValueError("Each triplet must contain exactly 3 points")
        total_distance += calculate_distance(*triplet)
    return total_distance
