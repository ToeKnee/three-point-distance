from math import radians, sin, cos, sqrt, atan2

class Point:
    def __init__(self, latitude, longitude):
        if not (-90 <= latitude <= 90):
            raise ValueError(f"Latitude {latitude} is out of bounds")
        if not (-180 <= longitude <= 180):
            raise ValueError(f"Longitude {longitude} is out of bounds")
        self.latitude = latitude
        self.longitude = longitude

    def distance(self, other):
        """Calculate the distance between two points using Haversine formula"""

        R = 6371.0  # Radius of the Earth in kilometers
        lat1, lon1 = radians(self.latitude), radians(self.longitude)
        lat2, lon2 = radians(other.latitude), radians(other.longitude)
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
