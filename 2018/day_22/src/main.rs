use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
  error::Error,
  fmt::Debug,
  io::stdin,
  ops::{Add, Index, IndexMut},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);

impl Point {
  fn non_negative(&self) -> bool {
    (self.0 >= 0) && (self.1 >= 0)
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Point(self.0 + other.0, self.1 + other.1)
  }
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
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
struct Pathing([Option<u32>; 3]);

impl Index<Tool> for Pathing {
  type Output = Option<u32>;

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
  costs: Pathing,
}

impl Region {
  fn new(erosion: u64) -> Self {
    Region {
      erosion,
      costs: Pathing([None, None, None]),
    }
  }

  // |cost| is considered cheaper if region has no costs for any tool
  // or if |cost| is lesser than or equal to any previous costs
  fn is_cheaper(&self, cost: u32) -> bool {
    self.costs.0.iter().all(|c| c.is_none())
      || self.costs.0.iter().any(|c| match c {
        Some(x) => cost <= *x,
        _ => false,
      })
  }

  fn is_cheaper_for_tool(&self, cost: u32, tool: Tool) -> bool {
    match self.costs[tool] {
      None => true,
      Some(c) => c > cost,
    }
  }
}

struct Map {
  depth: u16,
  target: Point,
  cells: HashMap<Point, Region>,
}

impl Map {
  // bounds to go beyond |target| when bounded_map feature is enabled.
  const BUFFER: i32 = 100;

  fn new(depth: u16, target: Point) -> Self {
    let cap = if cfg!(feature = "bounded_map") {
      (target.0 + Map::BUFFER) * (target.1 + Map::BUFFER)
    } else {
      target.0 * target.1
    } as usize;
    let mut m = Map {
      depth,
      target,
      cells: HashMap::<Point, Region>::with_capacity(cap),
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
    let pos_allowed = pos.non_negative()
      && match cfg!(feature = "bounded_map") {
        true => {
          let max_pos = self.target + Point(Map::BUFFER, Map::BUFFER);
          (pos.0 < max_pos.0) && (pos.1 < max_pos.1)
        }
        false => true,
      };
    match (pos_allowed, self.cells.contains_key(&pos)) {
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

  // Last step is unneeded as a priority queue is used; early short-circuit is
  // okay.  Enable bounded_map feature to short-circuit; it’s slow without this.

  let mouth = m.get_region(Point(0, 0)).unwrap();
  mouth.costs[Tool::Torch] = Some(0);
  let mut to_visit = BinaryHeap::new();
  to_visit.push(Reverse((0u32, Point(0, 0), Tool::Torch)));

  while let Some(Reverse((cost, pos, tool))) = to_visit.pop() {
    let region = RegionType::get(m.get_erosion(pos));
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
        let (new_cost, new_tool) =
          match RegionType::get(adj_region.erosion).is_allowed(tool) {
            true => (cost + 1, tool),
            false => (cost + 1 + 7, region.get_alternate(tool)),
          };
        if adj_pos == target {
          return new_cost + if new_tool != Tool::Torch { 7 } else { 0 };
        }
        // skip update if already visited with same tool at lesser cost
        if !adj_region.is_cheaper_for_tool(new_cost, new_tool) {
          continue;
        }
        // If already visited and this cost is cheaper, we will anyway start a
        // new follow-up by pushing to |to_visit|.
        adj_region.costs[new_tool] = Some(new_cost);
        // Don’t enlist in to_visit if we’ve already reached or if new cost
        // is cheaper than target’s current costs.
        if adj_pos != target
          && m.get_region(m.target).unwrap().is_cheaper(new_cost)
        {
          to_visit.push(Reverse((new_cost, adj_pos, new_tool)));
        }
      }
    }
  }
  m.get_region(target).unwrap().costs[Tool::Torch].unwrap()
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
