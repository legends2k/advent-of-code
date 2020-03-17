use std::io::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Point(i16, i16);

#[derive(Copy, Clone, Debug)]
struct Location {
  id: i32,
  pt: Point,
}

impl FromStr for Point {
  type Err = Error;
  fn from_str(s: &str) -> Result<Point, Error> {
    let coords: Vec<i16> = s
      .splitn(2, ',')
      .map(|s| s.trim().parse().unwrap_or_default())
      .collect();
    Ok(Point(coords[0], coords[1]))
  }
}

fn main() {
  let locs: Vec<Location> = io::stdin()
    .lock()
    .lines()
    .enumerate()
    .map(|(id, l)| Location {
      id: id as i32,
      pt: l.unwrap_or_default().parse().unwrap(),
    })
    .collect();

  let mut ids_x_sorted: Vec<usize> = (0..locs.len()).collect();
  ids_x_sorted.sort_unstable_by_key(|id| locs[*id].pt.0);
  let mut ids_y_sorted = ids_x_sorted.clone();
  ids_y_sorted.sort_unstable_by_key(|id| locs[*id].pt.1);

  // X → and Y ↓
  let left = locs[ids_x_sorted[0]].pt.0;
  let right = locs[*ids_x_sorted.last().unwrap()].pt.0;
  let top = locs[ids_y_sorted[0]].pt.1;
  let bottom = locs[*ids_y_sorted.last().unwrap()].pt.1;

  let width = (right - left + 1) as usize;
  let height = (bottom - top + 1) as usize;
}
