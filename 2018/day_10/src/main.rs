use std::error::Error;
use std::io::{self, BufRead};
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
struct Vec2(f32, f32);

impl std::str::FromStr for Vec2 {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(())
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  for l in io::stdin().lock().lines() {
    let line = l?;
    println!("{}", &line[10..24]);
    println!("{}", &line[36..42]);
    break;
  }
  Ok(())
}
