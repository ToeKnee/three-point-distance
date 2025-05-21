
class Point
  attr_accessor :latitude, :longitude

  def initialize(latitude, longitude)
    raise ArgumentError, 'Latitude must be between -90 and 90' unless latitude.between?(-90, 90)
    raise ArgumentError, 'Longitude must be between -180 and 180' unless longitude.between?(-180, 180)
    @latitude = latitude
    @longitude = longitude
  end

  # Calculate the distance between two points using Haversine formula
  def distance(other)
    # Radius of the Earth in kilometers
    r = 6371.0

    lat1 = to_radians(@latitude)
    lon1 = to_radians(@longitude)
    lat2 = to_radians(other.latitude)
    lon2 = to_radians(other.longitude)

    dlat = lat2 - lat1
    dlon = lon2 - lon1

    a = Math.sin(dlat / 2)**2 + Math.cos(lat1) * Math.cos(lat2) * Math.sin(dlon / 2)**2
    c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a))

    r * c
  end


end

def to_radians(value)
  value * Math::PI / 180.0
end

# Function to calculate the distance between three points
def calculate_distance(p1, p2, p3)
  p1.distance(p2) + p2.distance(p3)
end

# Function to calculate the total distance for a list of triplets
def calculate_total_distance(points)
  total_distance = 0.0
  points.each do |triplet|
    if triplet.length != 3
      puts "Error: triplet must contain exactly 3 points"
      return -1.0
    end
    total_distance += calculate_distance(triplet[0], triplet[1], triplet[2])
  end
  total_distance
end
