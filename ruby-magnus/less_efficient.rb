require 'json'
require './lib/distance'

# Load and parse JSON file
file_path = "../data/points-10000000.json"
# file_path = "../data/points-10000000.json"

puts "Parse JSON in Ruby, and create Point objects in Rust from Ruby. Calculate total distance in Rust."
puts "This means we move every point between Rust and Ruby, and then back again."
s = Time.now
file_contents = File.read(file_path)
parsed_data = JSON.parse(file_contents)
points = parsed_data.map do |triplet|
  triplet.map do |latitude, longitude|
    Point.new(latitude:, longitude:)
  end
end
puts "Loaded #{parsed_data.length} rows of 3 points from JSON in #{Time.now - s} seconds"

s = Time.now
total_distance = calculate_total_distance(points)
puts "Calculated total distance of #{total_distance} km in #{Time.now - s} seconds"
