# frozen_string_literal: true

require 'rspec'
require './distance'

RSpec.describe Point do
  describe '#initialize' do
    it 'initializes with latitude and longitude' do
      point = Point.new(latitude: 52.5200, longitude: 13.4050)
      expect(point.latitude).to eq(52.5200)
      expect(point.longitude).to eq(13.4050)
    end
  end

  describe '#distance' do
    it 'calculates the distance between two points' do
      point1 = Point.new(latitude: 52.5200, longitude: 13.4050) # Berlin
      point2 = Point.new(latitude: 48.8566, longitude: 2.3522)  # Paris
      expect(point1.distance(point2)).to be_within(0.1).of(877.463) # Distance in km
    end
  end

  describe '#calculate_distance' do
    it 'calculates the distance between three points' do
      point1 = Point.new(latitude: 45.0, longitude: 90.0)
      point2 = Point.new(latitude: 46.0, longitude: 91.0)
      point3 = Point.new(latitude: 47.0, longitude: 92.0)
      expect(calculate_distance(point1, point2, point3)).to be_within(0.1).of(270.7754) # Distance in km
    end
  end

  describe '#calculate_total_distance' do
    it 'calculates the total distance between an an array of three points' do
      point1 = Point.new(latitude: 45.0, longitude: 90.0)
      point2 = Point.new(latitude: 46.0, longitude: 91.0)
      point3 = Point.new(latitude: 47.0, longitude: 92.0)
      point4 = Point.new(latitude: 48.0, longitude: 93.0)
      point5 = Point.new(latitude: 49.0, longitude: 94.0)
      point6 = Point.new(latitude: 50.0, longitude: 95.0)
      expect(calculate_total_distance([
                                        [point1, point2, point3],
                                        [point4, point5, point6]
                                      ])).to be_within(0.1).of(536.74756) # Distance in km
    end
  end
end
