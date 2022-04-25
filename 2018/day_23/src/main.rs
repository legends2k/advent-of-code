use std::{
  cmp::Reverse,
  collections::BinaryHeap,
  error::Error,
  fmt::{self, Display, Formatter},
  io::{self, BufRead},
  ops::{Add, Sub},
  str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Point(i32, i32, i32);

// Manhattan/Taxicab (L1) distance
impl Sub for Point {
  type Output = u64;

  fn sub(self, other: Self) -> Self::Output {
    ((self.0 as i64 - other.0 as i64).abs()
      + (self.1 as i64 - other.1 as i64).abs()
      + (self.2 as i64 - other.2 as i64).abs()) as u64
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Point {
  fn add(self, delta: u32) -> Self {
    let d = delta as i32;
    Point(self.0 + d, self.1 + d, self.2 + d)
  }

  fn abs(self) -> Self {
    Point(self.0.abs(), self.1.abs(), self.2.abs())
  }

  fn max(self) -> i32 {
    *[self.0, self.1, self.2].iter().max().unwrap_or(&self.0)
  }

  fn sum(self) -> u64 {
    self.0 as u64 + self.1 as u64 + self.2 as u64
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
  radius: u32,
}

impl Bot {
  fn is_point_in_range(&self, p: Point) -> bool {
    (self.pos - p) <= self.radius as u64
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
  centre: Point,
  half_size: u32,
}

impl Cube {
  fn new(half_size: u32) -> Self {
    Cube {
      centre: Point(0, 0, 0),
      half_size,
    }
  }

  fn octants(&self) -> [Self; 8] {
    let half_size = self.half_size / 2;
    let hs = half_size as i32;
    [
      Cube {
        centre: self.centre + Point(hs, hs, hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(-hs, hs, hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(-hs, -hs, hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(hs, -hs, hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(hs, hs, -hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(-hs, hs, -hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(-hs, -hs, -hs),
        half_size,
      },
      Cube {
        centre: self.centre + Point(hs, -hs, -hs),
        half_size,
      },
    ]
  }

  fn is_in_range(&self, b: &Bot) -> bool {
    // in 2D: l1(c, p) <= 2 * (half_size + (b.radius / 2))
    (self.centre - b.pos) <= (3 * self.half_size + b.radius) as u64
  }

  fn distance_from_origin(&self) -> u64 {
    self.centre.abs().sum()
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut bots = Vec::<Bot>::with_capacity(1000);
  let mut max_coord: u32 = u32::MIN;
  let mut max_radius_bot_idx = 0;
  let mut max_radius = 0;
  for (i, l) in io::stdin().lock().lines().enumerate() {
    let b = Bot::from_str(&l?)?;
    if b.radius > max_radius {
      max_radius = b.radius;
      max_radius_bot_idx = i;
    }
    let possible_max_coord = b.pos.abs().add(b.radius).max() as u32;
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

  let cave = Cube::new(max_coord + 1);
  // max-heap but we want 2nd and 3rd tuple values be weighted by min; flip
  let mut heap = BinaryHeap::new();
  heap.push((
    bots.len(),
    Reverse(cave.half_size as i32),
    Reverse(cave.distance_from_origin() as i64),
    cave,
  ));
  while let Some((_, _, origin_distance, aabb)) = heap.pop() {
    if aabb.half_size == 0 {
      println!("Part 2: {}, {:?}", origin_distance.0, aabb.centre);
      break;
    }
    for oct in aabb.octants() {
      let bot_count = bots.iter().filter(|&bot| oct.is_in_range(bot)).count();
      heap.push((
        bot_count,
        Reverse(oct.half_size as i32),
        Reverse(oct.distance_from_origin() as i64),
        oct,
      ));
    }
  }

  Ok(())
}
