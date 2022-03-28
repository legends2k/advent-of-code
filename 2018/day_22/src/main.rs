use std::{
  collections::{HashMap, VecDeque},
  error::Error,
  fmt::{self, Debug, Formatter},
  io::stdin,
  mem,
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
  None,
  Gear,
  Torch,
  Unkown,
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

#[derive(Clone)]
struct Region {
  erosion: u64,
  // negative implies yet-to-reach
  cost: i32, // per-tool cost
  tool: Tool,
  from: Point,
}

impl Region {
  fn new(erosion: u64) -> Self {
    Region {
      erosion,
      cost: i32::MIN,
      tool: Tool::Unkown,
      from: Point(0, 0),
    }
  }
}

impl Debug for Region {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:?} \t {:?} \t {:?} \t {}",
      RegionType::get(self.erosion),
      self.from,
      self.tool,
      self.cost
    )?;
    Ok(())
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

fn shortest_path(m: &mut Map, target: Point) -> i32 {
  // Visit every adjacent cell
  //   Put cost-to-reach
  // Keep going in all 4 directions
  // Reach target
  // Drop yet-to-visit cells with higher cost
  // Complete all yet-to-visit cells with lower cost

  // Every node X has adjacent nodes Y1, Y2, ….  Every Y has two edges from X:
  //   1. With tool A
  //   2. With tool B
  // Shortest path from X to Y depends on the equiped tool when arriving at X.
  // However, once the cost and tool to arrive at X is known, the shortest edge
  // is eihter (1) or (2).  If in future a shorter route to X (with a different
  // tool) is found, we anyway move X to Open and revisit its descendants.

  let mut to_visit = VecDeque::<Point>::with_capacity(256);
  let mut visiting = VecDeque::<Point>::with_capacity(256);
  let mouth = m.get_region(Point(0, 0)).unwrap();
  mouth.tool = Tool::Torch;
  mouth.cost = 0;
  to_visit.push_back(Point(0, 0));
  let mut reached = false;
  let mut target_cost = i32::MIN;

  while !to_visit.is_empty() {
    mem::swap(&mut visiting, &mut to_visit);

    while let Some(pos) = visiting.pop_front() {
      let region = m.get_region(pos).unwrap().clone();
      // only pursue paths less costlier than current target cost if reached
      if reached && (region.cost >= target_cost) {
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
          let (mut cost, mut tool) =
            match RegionType::get(adj_region.erosion).is_allowed(region.tool) {
              true => ((region.cost + 1), region.tool),
              false => (
                (region.cost + 1 + 7),
                RegionType::get(region.erosion).get_alternate(region.tool),
              ),
            };
          // if target, include switching cost if not holding torch already
          if (adj_pos == target) && (adj_region.tool != Tool::Torch) {
            tool = Tool::Torch;
            cost += 7;
          }
          if pos == Point(4, 1) && adj_pos == Point(4, 2) {
            println!("{} -- {} ({:?})", adj_region.cost, cost, tool);
          }
          // update if already visited and has a lesser cost
          if (adj_region.cost >= 0) && (adj_region.cost < cost) {
            continue;
          }
          if pos == Point(4, 1) && adj_pos == Point(4, 2) {
            println!("here!");
          }
          // (adj_region.cost > 0) && (adj_region.cost > cost)
          // Not worrying about: already reached with greater cost as this
          // iteration will anyway start new follow-ups.
          adj_region.cost = cost;
          adj_region.tool = tool;
          adj_region.from = pos;
          if adj_pos == target {
            target_cost = cost;
            reached = true;
          } else if !reached || (cost < target_cost) {
            // pursue if target unreached or this cell’s cost is lesser
            to_visit.push_back(adj_pos);
          }
        }
      }
    }
  }
  target_cost
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
  dbg!(m.get_region(Point(1, 1)).unwrap());
  dbg!(m.get_region(Point(2, 1)).unwrap());
  dbg!(m.get_region(Point(3, 1)).unwrap());
  dbg!(m.get_region(Point(4, 1)).unwrap());
  dbg!(m.get_region(Point(4, 2)).unwrap());
  dbg!(m.get_region(Point(4, 3)).unwrap());
  dbg!(m.get_region(Point(4, 4)).unwrap());
  dbg!(m.get_region(Point(4, 5)).unwrap());
  dbg!(m.get_region(Point(4, 6)).unwrap());
  dbg!(m.get_region(Point(4, 7)).unwrap());
  dbg!(m.get_region(Point(4, 8)).unwrap());

  Ok(())
}
