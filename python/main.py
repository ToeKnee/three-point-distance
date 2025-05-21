import json
from datetime import datetime
from distance import Point, calculate_total_distance

def main():
    # Load and parse JSON file
    file_path = "../data/points-1000000.json"
    # file_path = "../data/points-10000000.json"

    s = datetime.now()
    points = []
    with open(file_path, "r") as file:
        file_contents = file.read()
        parsed_data = json.loads(file_contents)
        row_count = len(parsed_data)
        # Changing this from a list comprehension to a generator expression
        # increases performance massively, but is not a fair comparison to the
        # other examples which read everything into memory first,
        # then calculate the total distance.
        points = [
            [Point(*point) for point in triplet]
            for triplet in parsed_data
        ]
    e = datetime.now()
    time = (e - s).total_seconds()
    print(f"Loaded {row_count} rows of 3 points from JSON in {time} seconds")

    s = datetime.now()
    total_distance = calculate_total_distance(points)
    e = datetime.now()
    time = (e - s).total_seconds()
    print(f"Calculated total distance of {total_distance} km in {time} seconds")


if __name__ == "__main__":
    main()
