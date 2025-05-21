require 'rspec'
require './distance'

RSpec.describe Point do
  describe '#initialize' do
    it 'initializes with latitude and longitude' do
      point = Point.new(52.5200, 13.4050)
      expect(point.latitude).to eq(52.5200)
      expect(point.longitude).to eq(13.4050)
    end
  end

  describe '#distance' do
    it 'calculates the distance between two points' do
      point1 = Point.new(52.5200, 13.4050) # Berlin
      point2 = Point.new(48.8566, 2.3522)  # Paris
      expect(point1.distance(point2)).to be_within(0.1).of(877.463) # Distance in km
    end
  end

  describe '#calculate_distance' do
    it 'calculates the distance between three points' do
      point1 = Point.new(45.0, 90.0)
      point2 = Point.new(46.0, 91.0)
      point3 = Point.new(47.0, 92.0)
      expect(calculate_distance(point1, point2, point3)).to be_within(0.1).of(270.7754) # Distance in km
    end
  end

  describe '#calculate_total_distance' do
    it 'calculates the total distance between an an array of three points' do
      point1 = Point.new(45.0, 90.0)
      point2 = Point.new(46.0, 91.0)
      point3 = Point.new(47.0, 92.0)
      point4 = Point.new(48.0, 93.0)
      point5 = Point.new(49.0, 94.0)
      point6 = Point.new(50.0, 95.0)
      expect(calculate_total_distance([
        [point1, point2, point3],
        [point4, point5, point6]
      ])).to be_within(0.1).of(536.74756) # Distance in km
    end
  end

end
