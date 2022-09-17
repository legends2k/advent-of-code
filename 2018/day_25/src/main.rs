use std::{
  error::Error,
  io::{self, BufRead},
  ops::Sub,
  str::FromStr,
};

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
  id: u32,
  stars: Vec<Point>,
}

fn main() -> Result<(), Box<dyn Error>> {
  let points: Vec<Point> = io::stdin()
    .lock()
    .lines()
    .filter_map(|l| l.ok()?.parse().ok())
    .collect();

  for p in points {
    println!("{:?}", p);
  }

  Ok(())
}
