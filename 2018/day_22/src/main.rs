use std::collections::HashMap;
use std::error::Error;
use std::io::stdin;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point(i32, i32);

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RegionType {
  Rocky,
  Wet,
  Narrow,
}

struct Map {
  depth: u16,
  width: i32,
  height: i32,
  erosion: HashMap<Point, u64>,
}

impl Map {
  fn new(depth: u16, width: i32, height: i32) -> Self {
    let mut m = Map {
      depth,
      width,
      height,
      erosion: HashMap::<Point, u64>::with_capacity(
        (width * height * 2) as usize,
      ),
    };
    m.erosion.insert(Point(0, 0), depth as u64 % 20183);
    m.erosion.insert(Point(width, height), depth as u64 % 20183);
    m
  }

  fn get_erosion(&mut self, pt: Point) -> u64 {
    match self.erosion.get(&pt) {
      Some(&x) => x,
      None => {
        let depth = self.depth as u64;
        let e = match pt {
          Point(0, y) => (y as u64 * 48271 + depth) % 20183,
          Point(x, 0) => (x as u64 * 16807 + depth) % 20183,
          _ => {
            let left = Point(pt.0 - 1, pt.1);
            let up = Point(pt.0, pt.1 - 1);
            (self.get_erosion(left) * self.get_erosion(up) + depth) % 20183
          }
        };
        self.erosion.insert(pt, e);
        e
      }
    }
  }

  fn get_type(&mut self, pt: Point) -> RegionType {
    let erosion = self.get_erosion(pt);
    match erosion % 3 {
      0 => RegionType::Rocky,
      1 => RegionType::Wet,
      2 => RegionType::Narrow,
      _ => unreachable!(),
    }
  }

  fn risk_level(&mut self) -> u32 {
    let width = self.width;
    let height = self.height;
    (0..=width)
      .flat_map(|x| (0..=height).map(move |y| Point(x, y)))
      .map(|pt| self.get_type(pt) as u32)
      .sum()
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut buf = String::with_capacity(20);
  stdin().read_line(&mut buf)?;
  let depth = buf
    .trim_end()
    .rsplit_once(' ')
    .ok_or("Invalid input")?
    .1
    .parse::<u16>()?;
  buf.clear();
  stdin().read_line(&mut buf)?;
  let target = buf
    .trim_end()
    .rsplit_once(' ')
    .ok_or("Invalid input")?
    .1
    .split_once(',')
    .ok_or("Invalid input")?;
  let target_pos = Point(target.0.parse()?, target.1.parse()?);

  let mut m = Map::new(depth, target_pos.0, target_pos.1);
  // Part 1: print risk level of rectangle from cave mouth to target
  println!("Risk level: {}", m.risk_level());

  Ok(())
}
