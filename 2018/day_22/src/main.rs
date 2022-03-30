use std::{
  collections::{HashMap, VecDeque},
  error::Error,
  fmt::Debug,
  io::stdin,
  mem,
  ops::{Index, IndexMut},
};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point(i32, i32);

impl Point {
  fn is_valid(&self) -> bool {
    (self.0 >= 0) && (self.1 >= 0)
  }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tool {
  Gear,
  Torch,
  None,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RegionType {
  Rocky,
  Wet,
  Narrow,
}

impl RegionType {
  fn get(erosion: u64) -> Self {
    match erosion % 3 {
      0 => RegionType::Rocky,
      1 => RegionType::Wet,
      2 => RegionType::Narrow,
      _ => unreachable!(),
    }
  }

  fn is_allowed(self, t: Tool) -> bool {
    matches!(
      (self, t),
      (RegionType::Rocky, Tool::Gear)
        | (RegionType::Rocky, Tool::Torch)
        | (RegionType::Wet, Tool::Gear)
        | (RegionType::Wet, Tool::None)
        | (RegionType::Narrow, Tool::Torch)
        | (RegionType::Narrow, Tool::None)
    )
  }

  fn get_alternate(self, t: Tool) -> Tool {
    match (self, t) {
      (RegionType::Rocky, Tool::Gear) => Tool::Torch,
      (RegionType::Rocky, Tool::Torch) => Tool::Gear,
      (RegionType::Wet, Tool::Gear) => Tool::None,
      (RegionType::Wet, Tool::None) => Tool::Gear,
      (RegionType::Narrow, Tool::Torch) => Tool::None,
      (RegionType::Narrow, Tool::None) => Tool::Torch,
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug)]
struct Path {
  cost: Option<u32>,
  from: Point,
}

impl Path {
  fn new(from: Point) -> Self {
    Path { cost: None, from }
  }
}

#[derive(Clone, Debug)]
struct Pathing([Path; 3]);

impl Index<Tool> for Pathing {
  type Output = Path;

  fn index(&self, idx: Tool) -> &Self::Output {
    &self.0[idx as usize]
  }
}

impl IndexMut<Tool> for Pathing {
  fn index_mut(&mut self, idx: Tool) -> &mut Self::Output {
    &mut self.0[idx as usize]
  }
}

#[derive(Clone, Debug)]
struct Region {
  erosion: u64,
  pathing: Pathing,
}

impl Region {
  fn new(erosion: u64) -> Self {
    Region {
      erosion,
      pathing: Pathing([
        Path::new(Point(-1, -1)),
        Path::new(Point(-1, -1)),
        Path::new(Point(-1, -1)),
      ]),
    }
  }

  // |cost| is considered cheaper if region has no costs for any tool
  // or if no cost is greater than |cost|
  fn is_cheaper(&self, cost: u32) -> bool {
    self.pathing.0.iter().all(|p| p.cost.is_none())
      || self.pathing.0.iter().any(|p| match p.cost {
        Some(c) => cost <= c,
        _ => false,
      })
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

  fn risk_level(&mut self) -> u32 {
    let width = self.target.0;
    let height = self.target.1;
    (0..=width)
      .flat_map(|x| (0..=height).map(move |y| Point(x, y)))
      .map(|pt| RegionType::get(self.get_erosion(pt)) as u32)
      .sum()
  }

  fn get_region(&mut self, pos: Point) -> Option<&mut Region> {
    match (pos.is_valid(), self.cells.contains_key(&pos)) {
      (true, true) => self.cells.get_mut(&pos),
      (true, false) => {
        self.get_erosion(pos); // this inserts <Point, Region> into |self.cells|
        self.cells.get_mut(&pos)
      }
      _ => None,
    }
  }
}

fn shortest_path(m: &mut Map, target: Point) -> u32 {
  // Visit every adjacent cell
  //   Put cost-to-reach
  // Keep going in all 4 directions
  // Reach target
  // Drop yet-to-visit cells with higher cost
  // Complete all yet-to-visit cells with lower cost

  let mut to_visit = VecDeque::<(Point, Tool)>::with_capacity(256);
  let mut visiting = VecDeque::<(Point, Tool)>::with_capacity(256);
  let mouth = m.get_region(Point(0, 0)).unwrap();
  mouth.pathing[Tool::Torch].cost = Some(0);
  mouth.pathing[Tool::Torch].from = Point(-1, -1);
  to_visit.push_back((Point(0, 0), Tool::Torch));

  while !to_visit.is_empty() {
    mem::swap(&mut visiting, &mut to_visit);

    while let Some((pos, tool)) = visiting.pop_front() {
      let region = m.get_region(pos).unwrap().clone();
      let cost = region.pathing[tool].cost.expect("Should be some cost");

      // only pursue paths less costlier than current target cost if reached
      if !m.get_region(m.target).unwrap().is_cheaper(cost) {
        continue;
      }
      let adjs = [
        Point(pos.0 - 1, pos.1),
        Point(pos.0 + 1, pos.1),
        Point(pos.0, pos.1 - 1),
        Point(pos.0, pos.1 + 1),
      ];
      for adj_pos in adjs {
        if let Some(adj_region) = m.get_region(adj_pos) {
          let (mut new_cost, mut new_tool) =
            match RegionType::get(adj_region.erosion).is_allowed(tool) {
              true => (cost + 1, tool),
              false => (
                cost + 1 + 7,
                RegionType::get(region.erosion).get_alternate(tool),
              ),
            };
          // if target region include torch-switch cost if not holding it
          if adj_pos == target && new_tool != Tool::Torch {
            new_tool = Tool::Torch;
            new_cost += 7;
          }
          // skip update if already visited and has lesser cost
          if !adj_region.is_cheaper(new_cost) {
            continue;
          }
          // (adj_region.cost > 0) && (adj_region.cost > cost) Not worrying
          // about the situation where we’ve found a better solution as will
          // anyway start new follow-up by pushing to |to_visit|.
          adj_region.pathing[new_tool].cost = Some(new_cost);
          adj_region.pathing[new_tool].from = pos;
          // don’t enlist in to_visit if we’ve already reached or if new cost
          // is cheaper than target’s current costs
          if adj_pos != target
            && m.get_region(m.target).unwrap().is_cheaper(new_cost)
          {
            // pursue if target unreached or this cell’s cost is lesser
            to_visit.push_back((adj_pos, new_tool));
          }
        }
      }
    }
  }
  m.get_region(m.target)
    .unwrap()
    .pathing
    .0
    .iter()
    .min_by_key(|&p| p.cost.unwrap_or(u32::MAX))
    .unwrap()
    .cost
    .unwrap()
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

  println!(
    "Fastest path to target: {}",
    shortest_path(&mut m, target_pos)
  );

  Ok(())
}
