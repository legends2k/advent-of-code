use std::{
  cmp::{Ordering, Reverse},
  collections::BinaryHeap,
  error::Error,
  fmt::{self, Display, Formatter},
  io::{self, BufRead},
  ops::{Add, Sub},
  str::FromStr,
};

#[derive(Debug, Copy, Clone)]
struct Point(f64, f64, f64);

// Manhattan/Taxicab (L1) distance
impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Point {
  fn add(self, d: f64) -> Self {
    Point(self.0 + d, self.1 + d, self.2 + d)
  }

  fn abs(self) -> Self {
    Point(self.0.abs(), self.1.abs(), self.2.abs())
  }

  fn max(self) -> f64 {
    if self.0 >= self.1 && self.0 >= self.2 {
      self.0
    } else if self.1 >= self.0 && self.1 >= self.2 {
      self.1
    } else {
      self.2
    }
  }

  // Sum of all basis components
  fn sum(self) -> f64 {
    self.0 + self.1 + self.2
  }

  fn round(self) -> Self {
    Point(self.0.round(), self.1.round(), self.2.round())
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    ((self.0 - other.0).abs() < 0.5)
      && ((self.1 - other.1).abs() < 0.5)
      && ((self.2 - other.2).abs() < 0.5)
  }
}
impl Eq for Point {}
impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.0.to_bits(), self.1.to_bits(), self.2.to_bits()).cmp(&(
      other.0.to_bits(),
      other.1.to_bits(),
      other.2.to_bits(),
    ))
  }
}
impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {})", self.0, self.1, self.2)
  }
}

#[derive(Debug, Clone)]
struct Bot {
  pos: Point,
  radius: f64,
}

impl Bot {
  fn is_point_in_range(&self, p: Point) -> bool {
    (self.pos - p).abs().sum() <= self.radius
  }
}

impl FromStr for Bot {
  type Err = Box<dyn Error>;

  fn from_str(input: &str) -> Result<Bot, Self::Err> {
    let tok = input[5..].split(&[',', '>', ' ']).collect::<Vec<&str>>();
    let pos = Point(tok[0].parse()?, tok[1].parse()?, tok[2].parse()?);
    let radius = tok[5][2..].parse()?;
    Ok(Bot { pos, radius })
  }
}

// Using half size for an octant will lead to off-by-one errors; either |centre|
// and/or |half_size| should be floating point.
//
//           |  |  |
//          -1  0  1
//
// Center 0, half size 1 AABB can’t be split with just integers.

struct Aabb {
  centre: Point,
  half_size: f64,
}

impl PartialEq for Aabb {
  fn eq(&self, other: &Self) -> bool {
    self.to_bits().eq(&other.to_bits())
  }
}
impl Eq for Aabb {}
impl Ord for Aabb {
  fn cmp(&self, other: &Self) -> Ordering {
    self.to_bits().cmp(&other.to_bits())
  }
}
impl PartialOrd for Aabb {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.to_bits().partial_cmp(&other.to_bits())
  }
}

impl Aabb {
  fn to_bits(&self) -> [u64; 4] {
    [
      self.half_size.to_bits(),
      self.centre.0.to_bits(),
      self.centre.1.to_bits(),
      self.centre.2.to_bits(),
    ]
  }

  fn new(half_size: f64) -> Self {
    Aabb {
      centre: Point(0.0, 0.0, 0.0),
      half_size,
    }
  }

  fn octants(&self) -> [Self; 8] {
    let mut hs = self.half_size / 2.0;
    if hs.fract() < 0.5 {
      hs = hs.round();
    }
    [
      Aabb {
        centre: self.centre + Point(hs, hs, hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(-hs, hs, hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(-hs, -hs, hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(hs, -hs, hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(hs, hs, -hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(-hs, hs, -hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(-hs, -hs, -hs),
        half_size: hs,
      },
      Aabb {
        centre: self.centre + Point(hs, -hs, -hs),
        half_size: hs,
      },
    ]
  }

  fn has_bot_range(&self, b: &Bot) -> bool {
    let low = self.centre.add(-self.half_size);
    let high = self.centre.add(self.half_size);
    let dist = (b.pos - low).abs().sum() + (b.pos - high).abs().sum();
    (dist * 0.5) <= ((3.0 * self.half_size) + b.radius)

    // dist(centre.x, pos.x) <= half_size + (radius / 3)
    // dist(centre.y, pos.y) <= half_size + (radius / 3)
    // dist(centre.z, pos.z) <= half_size + (radius / 3)

    // in 2D: l1(c, p) <= 2 * (half_size + (b.radius / 2))
    // (self.centre - b.pos).abs().sum() <= ((3.0 * self.half_size) + b.radius)
  }

  fn distance_from_origin(&self) -> f64 {
    self.centre.abs().sum()
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut bots = Vec::<Bot>::with_capacity(1000);
  let mut max_coord = f64::MIN;
  let mut max_radius_bot_idx = 0;
  let mut max_radius = 0.0;
  for (i, l) in io::stdin().lock().lines().enumerate() {
    let b = Bot::from_str(&l?)?;
    if b.radius > max_radius {
      max_radius = b.radius;
      max_radius_bot_idx = i;
    }
    let possible_max_coord = b.pos.abs().add(b.radius).max();
    if possible_max_coord > max_coord {
      max_coord = possible_max_coord;
    }
    bots.push(b);
  }
  let max_bot = bots[max_radius_bot_idx].clone();

  println!(
    "Count of nanobots in range to strongest signal bot: {}",
    bots
      .iter()
      .filter(|&b| max_bot.is_point_in_range(b.pos))
      .count()
  );
  let cave = Aabb::new(max_coord);
  // Minimize size of AABB, distance to origin.  Reverse field 2, 3 on max-heap.
  let mut heap = BinaryHeap::new();
  heap.push((
    bots.len(),
    Reverse(cave.half_size.to_bits()),
    Reverse(cave.distance_from_origin().to_bits()),
    cave,
  ));
  let mut i = 0;
  while let Some((bc, _, _, aabb)) = heap.pop() {
    i += 1;
    if aabb.half_size < 1.0 {
      println!(
        "Distance to most populated point: {}",
        aabb.centre.round().abs().sum()
      );
      println!("Iteration count: {}, Bot count: {}", i, bc);
      break;
    }
    for oct in aabb.octants() {
      let bot_count = bots.iter().filter(|&bot| oct.has_bot_range(bot)).count();
      heap.push((
        bot_count,
        Reverse(oct.half_size.to_bits()),
        Reverse(oct.distance_from_origin().to_bits()),
        oct,
      ));
    }
  }

  Ok(())
}
