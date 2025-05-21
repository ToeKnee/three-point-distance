function Point(latitude, longitude) {
  if (latitude < -90 || latitude > 90) {
    throw new Error("Latitude must be between -90 and 90 degrees");
  }
  if (longitude < -180 || longitude > 180) {
    throw new Error("Longitude must be between -180 and 180 degrees");
  }
  return { latitude, longitude };
}

// Calculate the distance between two points using the Haversine formula.
function distance(first, second) {
  const lat1 = (first.latitude * Math.PI) / 180;
  const lat2 = (second.latitude * Math.PI) / 180;
  const dLat = ((second.latitude - first.latitude) * Math.PI) / 180;
  const dLon = ((second.longitude - first.longitude) * Math.PI) / 180;
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) + Math.cos(lat1) * Math.cos(lat2) * Math.sin(dLon / 2) * Math.sin(dLon / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  const R = 6371; // Radius of the Earth in kilometers
  return R * c; // Distance in kilometers
}

function calculate_distance(point1, point2, point3) {
  return distance(point1, point2) + distance(point2, point3);
}

function calculate_total_distance(points) {
  let totalDistance = 0;
  for (let i = 0; i < points.length; i++) {
    let triplet = points[i];
    if (triplet.length != 3) {
      console.error("Triplet length is not 3");
      continue;
    }
    const point1 = Point(triplet[0][0], triplet[0][1]);
    const point2 = Point(triplet[1][0], triplet[1][1]);
    const point3 = Point(triplet[2][0], triplet[2][1]);
    totalDistance += calculate_distance(point1, point2, point3);
  }
  return totalDistance;
}

async function loadPoints() {
  const start = Date.now();
  console.warn("Max load of 1000000 points - larger amounts won't load");
  const data = await fetch("http://127.0.0.1:8000/points-1000000.json");
  const points = await data.json();
  const end = Date.now();
  console.log(`Loaded ${points.length} rows of 3 points from JSON in ${(end - start) / 1000} seconds`);
  return points;
}

async function main() {
  const points = await loadPoints();

  let start = Date.now();
  let total_distance = calculate_total_distance(points);
  let end = Date.now();
  console.log(`Calculated total distance of ${total_distance} km in ${(end - start) / 1000} seconds`);

  start = Date.now();
  total_distance = 0;
  for (let i = 0; i < 10; i++) {
    total_distance += calculate_total_distance(points);
  }
  end = Date.now();
  console.log(`Calculated total distance of ${total_distance} km in ${(end - start) / 1000} seconds - 10 times`);
}

main();
