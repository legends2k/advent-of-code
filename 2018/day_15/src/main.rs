use core::fmt;
use std::{
  cmp::Ordering,
  error::Error,
  fmt::Display,
  io::{stdin, BufRead},
  mem,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Point(u8, u8);

impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.1 != other.1 {
      true => self.1.cmp(&other.1),
      false => self.0.cmp(&other.0),
    }
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FighterKind {
  Elf,
  Goblin,
}

#[derive(Debug, Eq, PartialEq)]
struct Fighter {
  kind: FighterKind,
  pos: Point,
}

impl Ord for Fighter {
  fn cmp(&self, other: &Self) -> Ordering {
    self.pos.cmp(&other.pos)
  }
}

impl PartialOrd for Fighter {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.pos.cmp(&other.pos))
  }
}

impl Fighter {
  fn target(&self, map: &Map) -> Option<Point> {
    [
      Point(self.pos.0, self.pos.1 - 1),
      Point(self.pos.0 - 1, self.pos.1),
      Point(self.pos.0 + 1, self.pos.1),
      Point(self.pos.0, self.pos.1 + 1),
    ]
    .iter()
    .find(|&&pt| match map.cell(pt) {
      Some(Cell::Occupied { kind, .. }) if kind != self.kind => true,
      _ => false,
    })
    .map(|&pt| pt)
  }
}

#[derive(Copy, Clone)]
enum Cell {
  Wall,
  Vacant { previous: Point, dist: u16 },
  Occupied { kind: FighterKind },
}

impl Cell {
  const MAX: u16 = u16::MAX;

  fn is_vacant(&self) -> bool {
    matches!(self, Cell::Vacant { .. })
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let symbol = match self {
      Cell::Wall => b'#',
      // Destructuring structs when matching
      // https://stackoverflow.com/q/41390457/183120
      Cell::Vacant { .. } => b'.',
      Cell::Occupied {
        kind: FighterKind::Elf,
        ..
      } => b'E',
      Cell::Occupied {
        kind: FighterKind::Goblin,
        ..
      } => b'G',
    };
    write!(f, "{}", symbol as char)
  }
}

// https://stackoverflow.com/a/62101709/183120
fn clearscreen(f: &mut fmt::Formatter<'_>) -> fmt::Result {
  write!(f, "\x1B[2J\x1B[1;1H")
}

const MAP_DIMENSION_MAX: usize = 32;

struct Map {
  layout: Vec<Cell>,
  fighters: Vec<Fighter>,
  width: usize,
  height: usize,
}

impl Map {
  fn point_to_idx(&self, pt: Point) -> usize {
    self.width * pt.1 as usize + pt.0 as usize
  }

  fn cell(&self, pt: Point) -> Option<Cell> {
    if pt.0 >= self.width as u8 || pt.1 >= self.height as u8 {
      return None;
    }
    Some(self.layout[self.point_to_idx(pt)])
  }

  fn is_vacant(&self, pt: Point) -> bool {
    match self.cell(pt) {
      Some(x) => x.is_vacant(),
      None => false,
    }
  }

  fn targets(&self, from: Point) -> Vec<Point> {
    let cell = &self.layout[self.point_to_idx(from)];
    let enemy_kind = match cell {
      Cell::Occupied {
        kind: FighterKind::Elf,
        ..
      } => FighterKind::Goblin,
      Cell::Occupied {
        kind: FighterKind::Goblin,
        ..
      } => FighterKind::Elf,
      _ => panic!("Invalid source point!"),
    };
    self
      .fighters
      .iter()
      .filter(|&fighter| fighter.kind == enemy_kind)
      .flat_map(|fighter| {
        [
          Point(fighter.pos.0, fighter.pos.1 - 1),
          Point(fighter.pos.0 - 1, fighter.pos.1),
          Point(fighter.pos.0 + 1, fighter.pos.1),
          Point(fighter.pos.0, fighter.pos.1 + 1),
        ]
        .iter()
        .filter(|&&pt| self.is_vacant(pt))
        .map(|&pt| pt)
        .collect::<Vec<_>>()
      })
      .collect()
  }

  /** Reset distances stored in vacant cells to default */
  fn clear(&mut self) {
    self
      .layout
      .iter_mut()
      .filter(|c| c.is_vacant())
      .for_each(|c| {
        *c = Cell::Vacant {
          previous: Point::default(),
          dist: Cell::MAX,
        }
      });
  }

  fn set(&mut self, p: Point, cell: Cell) {
    let idx = self.point_to_idx(p);
    self.layout[idx] = cell;
  }

  /** Deduce possible next step for unit at `src` */
  fn next_step(&mut self, src: Point, dsts: &[Point]) -> Option<Point> {
    if dsts.is_empty() {
      return None;
    }
    self.clear();
    // loop until all vacancies are visited or one of |dsts| is reached
    let mut visiting = Vec::<(Point, Point)>::with_capacity(256);
    let mut to_visit = Vec::<(Point, Point)>::with_capacity(256);
    // order flipped since Vec::{push, pop} is FILO
    to_visit.push((Point(src.0, src.1 + 1), src));
    to_visit.push((Point(src.0 + 1, src.1), src));
    to_visit.push((Point(src.0 - 1, src.1), src));
    to_visit.push((Point(src.0, src.1 - 1), src));
    let mut cur_dist = 0;
    let mut final_dst: Option<Point> = None;
    let mut almost_reached = false;

    while !to_visit.is_empty() {
      mem::swap(&mut visiting, &mut to_visit);
      to_visit.clear();
      cur_dist += 1;
      while let Some((pt, from)) = visiting.pop() {
        let cell = self.cell(pt); // proceed only if cell is vacant
        if let Some(Cell::Vacant { previous, dist }) = cell {
          // if path shorter or if same distance tie break with previous point
          if dist > cur_dist || ((dist == cur_dist) && (previous > from)) {
            self.set(
              pt,
              Cell::Vacant {
                previous: from,
                dist: cur_dist,
              },
            );
            if dsts.iter().any(|&p| p == pt) {
              final_dst = match final_dst {
                None => {
                  // Reached first target; stop futher outer loop iterations as
                  // |cur_dist| only increases; we won’t find a closer target.
                  // Process inner loop to completion since another target with
                  // same dist but preceding |pt| in reading order may be found.
                  almost_reached = true;
                  to_visit.clear();
                  println!("None: Setting final dst: {:?}", pt);
                  Some(pt)
                }
                Some(old_dst) => match pt < old_dst {
                  true => Some(pt),
                  false => final_dst,
                },
              };
            }
            if !almost_reached {
              to_visit.push((Point(pt.0, pt.1 + 1), pt));
              to_visit.push((Point(pt.0 + 1, pt.1), pt));
              to_visit.push((Point(pt.0 - 1, pt.1), pt));
              to_visit.push((Point(pt.0, pt.1 - 1), pt));
            }
          }
        }
      }
    }

    // if found a path, backtrack and choose the optimal next step
    match final_dst {
      Some(mut pt) => {
        loop {
          match self.cell(pt) {
            Some(Cell::Vacant { previous, .. }) if self.is_vacant(previous) => {
              pt = previous
            }
            Some(Cell::Vacant { .. }) => break,
            _ => panic!("Unexpected state"),
          }
        }
        Some(pt)
      }
      None => None,
    }
  }

  fn move_fighter(&mut self, fighter_idx: usize, pt: Point) {
    let old_idx = self.point_to_idx(self.fighters[fighter_idx].pos);
    self.layout[old_idx] = Cell::Vacant {
      previous: Point::default(),
      dist: Cell::MAX,
    };
    self.fighters[fighter_idx].pos = pt;
    let new_idx = self.point_to_idx(pt);
    self.layout[new_idx] = Cell::Occupied {
      kind: self.fighters[fighter_idx].kind,
    };
  }
}

impl Display for Map {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    clearscreen(f)?;
    for row in 0..self.height {
      for col in 0..self.width {
        write!(f, "{}", self.layout[row as usize * self.width + col])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut layout =
    Vec::<Cell>::with_capacity(MAP_DIMENSION_MAX * MAP_DIMENSION_MAX);
  let mut fighters = Vec::<Fighter>::with_capacity(32);

  let mut height: usize = 0;
  for l in stdin().lock().lines() {
    let line = l?;
    let mut digest: Vec<Cell> = line
      .bytes()
      .enumerate()
      .map(|(idx, symbol)| match symbol {
        b'#' => Cell::Wall,
        b'E' => {
          fighters.push(Fighter {
            kind: FighterKind::Elf,
            pos: Point(idx as u8, height as u8),
          });
          Cell::Occupied {
            kind: FighterKind::Elf,
          }
        }
        b'G' => {
          fighters.push(Fighter {
            kind: FighterKind::Goblin,
            pos: Point(idx as u8, height as u8),
          });
          Cell::Occupied {
            kind: FighterKind::Goblin,
          }
        }
        _ => Cell::Vacant {
          previous: Point::default(),
          dist: Cell::MAX,
        },
      })
      .collect();
    layout.append(&mut digest);
    height += 1;
  }
  let width = layout.len() / height;

  let mut map = Map {
    layout,
    fighters,
    width,
    height,
  };

  for _ in 0..3 {
    // fix turn order amongst fighters
    map.fighters.sort();
    for idx in 0..map.fighters.len() {
      // attack or move?
      if let Some(_) = map.fighters[idx].target(&map) {
      } else {
        let targets = map.targets(map.fighters[idx].pos);
        if let Some(pt) = map.next_step(map.fighters[idx].pos, &targets) {
          map.move_fighter(idx, pt);
          println!("{}", map);
        }
      }
    }
  }

  Ok(())
}
