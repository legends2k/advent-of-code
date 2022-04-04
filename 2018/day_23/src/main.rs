use std::{
  error::Error,
  io::{self, BufRead},
  ops::Sub,
  str::FromStr,
};

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct Bot {
  pos: Point,
  radius: u32,
}

impl Bot {
  fn is_in_range(&self, p: Point) -> bool {
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

fn main() -> Result<(), Box<dyn Error>> {
  let mut bots = Vec::<Bot>::with_capacity(1000);
  let mut max_radius_bot_idx = 0;
  let mut max_radius = 0;
  for (i, l) in io::stdin().lock().lines().enumerate() {
    let b = Bot::from_str(&l?)?;
    if b.radius > max_radius {
      max_radius = b.radius;
      max_radius_bot_idx = i;
    }
    bots.push(b);
  }
  let max_bot = bots[max_radius_bot_idx].clone();

  println!(
    "Count of nanobots in range to strongest signal bot: {}",
    bots.iter().filter(|&b| max_bot.is_in_range(b.pos)).count()
  );

  Ok(())
}
