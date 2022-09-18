use std::{
  collections::HashMap,
  error::Error,
  io::{self, BufRead},
  ops::Sub,
  str::FromStr,
};

#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($( $args:expr ),*) => { print!( $( $args ),* ); }
}
#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
  ($( $args:expr ),*) => {};
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32, i32, i32);

impl Point {
  fn abs(self) -> Self {
    Point(self.0.abs(), self.1.abs(), self.2.abs(), self.3.abs())
  }

  fn sum(self) -> i32 {
    self.0 + self.1 + self.2 + self.3
  }

  // Manhattan/Taxicab distance
  fn dist(&self, other: Point) -> i32 {
    (*self - other).abs().sum()
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Point(
      self.0 - other.0,
      self.1 - other.1,
      self.2 - other.2,
      self.3 - other.3,
    )
  }
}

impl FromStr for Point {
  type Err = Box<dyn Error>;

  fn from_str(input: &str) -> Result<Point, Self::Err> {
    let parts = input.split(",").take(4).collect::<Vec<_>>();
    Ok(Point(
      parts[0].parse::<i32>()?,
      parts[1].parse::<i32>()?,
      parts[2].parse::<i32>()?,
      parts[3].parse::<i32>()?,
    ))
  }
}

struct Constellation {
  stars: Vec<Point>,
}

impl Constellation {
  fn is_connected(&self, p: Point) -> bool {
    self.stars.iter().any(|s| s.dist(p) <= 3)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let points: Vec<Point> = io::stdin()
    .lock()
    .lines()
    .filter_map(|l| l.ok()?.parse().ok())
    .collect();

  let mut id = 0;
  let mut forests = HashMap::<u32, Constellation>::with_capacity(points.len());
  for &p in points.iter() {
    dbg_print!("{:?}\n", p);
    let part_of: Vec<_> = forests
      .iter()
      .filter(|(_, f)| f.is_connected(p))
      .map(|(&id, _)| id)
      .collect();
    if part_of.is_empty() {
      // Union-Find’s makeSet operation
      forests.insert(id, Constellation { stars: vec![p] });
      dbg_print!("  New constellation: {id}\n");
      id += 1;
    } else {
      let base_const_idx = part_of[0];
      forests.get_mut(&base_const_idx).unwrap().stars.push(p);
      // Union-Find’s union operation
      dbg_print!("  Added to {base_const_idx}\n");
      for c in part_of.iter().skip(1) {
        dbg_print!("    Constellation merged: {c}\n");
        let mut constellation_to_kill = forests.remove(c).unwrap();
        forests
          .get_mut(&base_const_idx)
          .unwrap()
          .stars
          .append(&mut constellation_to_kill.stars);
      }
    }
  }
  println!("Constellations: {}", forests.len());

  Ok(())
}
