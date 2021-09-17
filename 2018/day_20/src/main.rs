use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read};
use std::mem;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl Point {
  fn climb(&self, delta: i32) -> Self {
    Point(self.0, self.1 + delta)
  }

  fn slide(&self, delta: i32) -> Self {
    Point(self.0 + delta, self.1)
  }
}

struct Map {
  data: Vec<u8>,
  width: usize,
  height: usize,
}

impl Map {
  fn new(dim: Point, pos: Point, input: &str) -> Self {
    let width = dim.0 as usize;
    let height = dim.1 as usize;
    let mut m = Map {
      data: vec![b'#'; width * height],
      width,
      height,
    };
    m.parse(pos, input);
    m
  }

  fn parse(&mut self, mut pos: Point, input: &str) {
    self.set(pos, b'X');
    let mut fork_points = Vec::with_capacity(128);
    for c in input
      .bytes()
      .skip_while(|&c| c != b'^')
      .skip(1)
      .take_while(|&c| c != b'$')
    {
      match c {
        b'N' => {
          self.set(pos.climb(-1), b'-');
          self.set(pos.climb(-2), b'.');
          pos = pos.climb(-2)
        }
        b'S' => {
          self.set(pos.climb(1), b'-');
          self.set(pos.climb(2), b'.');
          pos = pos.climb(2)
        }
        b'W' => {
          self.set(pos.slide(-1), b'|');
          self.set(pos.slide(-2), b'.');
          pos = pos.slide(-2)
        }
        b'E' => {
          self.set(pos.slide(1), b'|');
          self.set(pos.slide(2), b'.');
          pos = pos.slide(2)
        }
        b'(' => fork_points.push(pos),
        b'|' => pos = *fork_points.last().unwrap(),
        b')' => {
          fork_points.pop();
        }
        _ => unreachable!(),
      };
    }
  }

  fn set(&mut self, p: Point, value: u8) {
    // support `p` with negative values too; cast to i32 and back
    let idx = (self.width as i32 * p.1 + p.0) as usize;
    self.data[idx] = value;
  }

  fn get(&self, p: Point) -> u8 {
    let idx = (self.width as i32 * p.1 + p.0) as usize;
    self.data[idx]
  }

  fn is_reachable(&self, p: Point, dir: u8) -> Option<Point> {
    match dir {
      b'N' if self.get(p.climb(-1)) == b'-' => Some(p.climb(-2)),
      b'S' if self.get(p.climb(1)) == b'-' => Some(p.climb(2)),
      b'W' if self.get(p.slide(-1)) == b'|' => Some(p.slide(-2)),
      b'E' if self.get(p.slide(1)) == b'|' => Some(p.slide(2)),
      _ => None,
    }
  }

  fn visit_rooms(&self, pos: Point) -> HashMap<Point, u16> {
    let mut room_door_count =
      HashMap::<Point, u16>::with_capacity(self.width * self.height);
    let mut visiting = Vec::<Point>::with_capacity(256);
    let mut to_visit = Vec::<Point>::with_capacity(256);
    to_visit.extend(
      [b'N', b'E', b'W', b'S']
        .iter()
        .filter_map(|&dir| self.is_reachable(pos, dir))
        .collect::<Vec<Point>>(),
    );
    let mut cur_dist = 0;
    while !to_visit.is_empty() {
      mem::swap(&mut visiting, &mut to_visit);
      to_visit.clear();
      cur_dist += 1;
      while let Some(pt) = visiting.pop() {
        // skip if already visited
        if room_door_count.get(&pt).is_none() {
          room_door_count.insert(pt, cur_dist);
          to_visit.extend(
            [b'N', b'E', b'W', b'S']
              .iter()
              .filter_map(|&dir| self.is_reachable(pt, dir))
              .collect::<Vec<Point>>(),
          );
        }
      }
    }
    room_door_count
  }
}

impl Debug for Map {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for r in 0..self.height {
      let s: String = self.data[r * self.width..]
        .iter()
        .take(self.width)
        .map(|&c| char::from(c))
        .collect();
      writeln!(f, "{}", s)?
    }
    Ok(())
  }
}

fn compute_dims(input: &str) -> Result<(Point, Point), Box<dyn Error>> {
  let mut fork_points = Vec::with_capacity(128);
  let mut pos = Point(0, 0);
  let (mut min, mut max) =
    (Point(i32::MAX, i32::MAX), Point(-i32::MAX, -i32::MAX));
  for c in input
    .bytes()
    .skip_while(|&c| c != b'^')
    .skip(1)
    .take_while(|&c| c != b'$')
  {
    match c {
      b'N' => pos.1 -= 2,
      b'E' => pos.0 += 2,
      b'W' => pos.0 -= 2,
      b'S' => pos.1 += 2,
      b'(' => fork_points.push(pos),
      b'|' => pos = *fork_points.last().ok_or("Invalid input")?,
      b')' => {
        fork_points.pop();
      }
      _ => unreachable!("Invalid input"),
    }
    min.0 = min.0.min(pos.0);
    max.0 = max.0.max(pos.0);
    min.1 = min.1.min(pos.1);
    max.1 = max.1.max(pos.1);
  }
  // expand by 1 on all sides for the walls
  min.0 -= 1;
  min.1 -= 1;
  max.0 += 1;
  max.1 += 1;
  // + 1 as we want stops and not spans
  let dim_x = max.0 - min.0 + 1;
  let dim_y = max.1 - min.1 + 1;
  Ok((Point(dim_x, dim_y), Point(-min.0, -min.1)))
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut input = String::with_capacity(15 * 1024);
  // drop the trailing linefeed ‘\n’
  io::stdin().read_to_string(&mut input)?;
  input.pop();

  let (dims, pos) = compute_dims(&input)?;
  let m = Map::new(dims, pos, &input);

  // Part 1: farthest room with maximum doors to cross
  let rooms_doors = m.visit_rooms(pos);
  let farthest_room = rooms_doors
    .iter()
    .max_by(|&a, &b| a.1.cmp(b.1))
    .ok_or("No rooms.  Invalid input!")?;
  println!(
    "Room @ ({}, {}) farthest; doors in between: {}",
    farthest_room.0 .0, farthest_room.0 .1, farthest_room.1
  );

  // Part 2: rooms needing ≥ 1000 door to pass through
  println!(
    "Rooms needing to cross 1000+ doors: {}",
    rooms_doors
      .iter()
      .filter(|&(_, &dist)| dist >= 1000)
      .count()
  );

  Ok(())
}
