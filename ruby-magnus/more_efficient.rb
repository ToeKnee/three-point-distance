# frozen_string_literal: true

require 'json'
require './lib/distance'

# Load and parse JSON file
file_path = '../data/points-10000000.json'
# file_path = "../data/points-10000000.json"

puts 'Parse JSON in Ruby, but create Point objects in Rust. Calculate total distance in Rust.'
s = Time.now
file_contents = File.read(file_path)
parsed_data = JSON.parse(file_contents)
puts "Loaded #{parsed_data.length} rows of 3 points from JSON in #{Time.now - s} seconds"

s = Time.now
total_distance = calculate_total_distance_from_array(parsed_data)
puts "Calculated total distance of #{total_distance} km in #{Time.now - s} seconds"
