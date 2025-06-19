import json
from datetime import datetime
from fast_distance import Point, calculate_total_distance, calculate_total_distance_from_array

def main():
    print("Parse JSON in Python, but create Point objects in Rust. Calculate total distance in Rust.")
    # Load and parse JSON file
    file_path = "../data/points-10000000.json"
    # file_path = "../data/points-10000000.json"

    s = datetime.now()
    points = []
    with open(file_path, "r") as file:
        file_contents = file.read()
        points = json.loads(file_contents)
        row_count = len(points)
    e = datetime.now()
    time = (e - s).total_seconds()
    print(f"Loaded {row_count} rows of 3 points from JSON in {time} seconds")

    s = datetime.now()
    total_distance = calculate_total_distance_from_array(points)
    e = datetime.now()
    time = (e - s).total_seconds()
    print(f"Calculated total distance of {total_distance} km in {time} seconds")


if __name__ == "__main__":
    main()
