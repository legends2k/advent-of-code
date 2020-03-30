use std::io::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

type LocId = usize;
type Dist = i16;
type LocDist = (LocId, Dist);

#[derive(Copy, Clone, Debug)]
struct Point(Dist, Dist);

impl Point {
  fn taxicab_dist(&self, other: Point) -> Dist {
    ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as Dist
  }
}

impl FromStr for Point {
  type Err = Error;
  fn from_str(s: &str) -> Result<Point, Error> {
    let coords: Vec<Dist> = s
      .splitn(2, ',')
      .map(|s| s.trim().parse().unwrap_or_default())
      .collect();
    Ok(Point(coords[0], coords[1]))
  }
}

#[derive(Copy, Clone, Debug)]
struct Location {
  id: LocId,
  pt: Point,
}

struct Map {
  locs: Vec<Location>,
  left: Dist,
  top: Dist,
  right: Dist,
  bottom: Dist,
}

impl Map {
  // X → and Y ↓

  fn new(locs: Vec<Location>) -> Self {
    let left = locs
      .iter()
      .min_by_key(|&loc| loc.pt.0)
      .map(|&loc| loc.pt.0)
      .unwrap();
    let right = locs
      .iter()
      .max_by_key(|&loc| loc.pt.0)
      .map(|&loc| loc.pt.0)
      .unwrap();
    let top = locs
      .iter()
      .min_by_key(|&loc| loc.pt.1)
      .map(|&loc| loc.pt.1)
      .unwrap();
    let bottom = locs
      .iter()
      .max_by_key(|&loc| loc.pt.1)
      .map(|&loc| loc.pt.1)
      .unwrap();
    Map {
      locs,
      left,
      top,
      right,
      bottom,
    }
  }

  fn is_on_edge(&self, pt: Point) -> bool {
    (pt.0 == self.left)
      || (pt.0 == self.right)
      || (pt.1 == self.top)
      || (pt.1 == self.bottom)
  }
}

fn main() {
  let locs: Vec<Location> = io::stdin()
    .lock()
    .lines()
    .enumerate()
    .map(|(id, l)| Location {
      id: id,
      pt: l.unwrap_or_default().parse().unwrap(),
    })
    .collect();
  let map = Map::new(locs);

  const DIST_SUM_THRESHOLD: i16 = 10_000;
  let mut fav_region_size = 0u16;

  // map of loc id and count
  let mut loc_freq = vec![0.0f32; map.locs.len()];
  for row in map.top..=map.bottom {
    for col in map.left..=map.right {
      let pt = Point(col, row);
      let mut candidates: Vec<LocDist> = map
        .locs
        .iter()
        .map(|l| (l.id, l.pt.taxicab_dist(pt)))
        .collect();

      // part 1
      candidates.sort_unstable_by_key(|&(_id, dist)| dist);
      // if not a tie between two locations
      if candidates[0].1 != candidates[1].1 {
        if map.is_on_edge(pt) {
          loc_freq[candidates[0].0] = std::f32::INFINITY;
        } else {
          loc_freq[candidates[0].0] += 1.0;
        }
      }

      // part 2
      let dist_sum: i16 = candidates.iter().map(|(_id, dist)| *dist).sum();
      if dist_sum < DIST_SUM_THRESHOLD {
        fav_region_size += 1;
      }
    }
  }

  // check for maximum frequency; skip ∞ cases
  if let Some((id, freq)) = loc_freq
    .iter()
    .enumerate() // not filtering ±∞ as they get converted to 0
    .max_by_key(|(_id, &freq)| freq as u16)
  {
    println!("Location #{} has the largest area: {} spots", id, freq);
  }
  println!(
    "Spots with Σ taxicab distance < {}: {}",
    DIST_SUM_THRESHOLD, fav_region_size
  );
}
