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

struct Region {
  erosion: u64,
  cost: [i32; 2], // per-tool cost
}

impl Region {
  fn new(erosion: u64) -> Self {
    Region {
      erosion,
      cost: [i32::MIN; 2],
    }
  }
}

struct Map {
  depth: u16,
  target: Point,
  cells: HashMap<Point, Region>,
}

impl Map {
  fn new(depth: u16, target: Point) -> Self {
    let mut m = Map {
      depth,
      target,
      cells: HashMap::<Point, Region>::with_capacity(
        (target.0 * target.1 * 2) as usize,
      ),
    };
    m.cells
      .insert(Point(0, 0), Region::new(depth as u64 % 20183));
    m.cells.insert(target, Region::new(depth as u64 % 20183));
    m
  }

  fn get_erosion(&mut self, pt: Point) -> u64 {
    match self.cells.get(&pt) {
      Some(Region { erosion, .. }) => *erosion,
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
        self.cells.insert(pt, Region::new(e));
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
    let width = self.target.0;
    let height = self.target.1;
    (0..=width)
      .flat_map(|x| (0..=height).map(move |y| Point(x, y)))
      .map(|pt| self.get_type(pt) as u32)
      .sum()
  }
}

fn shortest_path_cost(m: &Map) -> u32 {
  // Go to adjacent cell with least cost
  //   Put cost-to-reach in each cell
  // Keep going in all 4 directions
  // Target reached
  // Drop yet-to-visit cells with higher cost
  // Complete all yet-to-visit cells with lower cost
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

  let mut m = Map::new(depth, target_pos);
  // Part 1: print risk level of rectangle from cave mouth to target
  println!("Risk level: {}", m.risk_level());

  Ok(())
}
