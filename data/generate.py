import itertools
import json
import random

def generate_lat_lng():
    lat = random.uniform(-90, 90)
    lng = random.uniform(-180, 180)
    return lat, lng


class SerializableGenerator(list):
    """Generator that is serializable by JSON"""

    def __init__(self, iterable):
        tmp_body = iter(iterable)
        try:
            self._head = iter([next(tmp_body)])
            self.append(tmp_body)
        except StopIteration:
            self._head = []

    def __iter__(self):
        return itertools.chain(self._head, *self[:1])

def main(n=100000000):
    data = (
            [generate_lat_lng(), generate_lat_lng(), generate_lat_lng()]
            for i in range(0, n)
    )
    with open(f"points-{n}.json", "w") as f:
        iter_json = json.JSONEncoder().iterencode(SerializableGenerator(data))
        for chunk in iter_json:
            f.write(chunk)


if __name__ == "__main__":
    times = [10, 100, 1000, 10000, 100000, 1000000, 10000000]
    for n in times:
        print(f"Generating {n} points")
        main(n)
        print(f"Finished generating {n} points")
        print()
