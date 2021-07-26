use core::fmt;
use std::{
  error::Error,
  fmt::Display,
  io::{stdin, BufRead},
  mem,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Point(u8, u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FighterKind {
  Elf,
  Goblin,
}

#[derive(Debug)]
struct Fighter {
  id: u8,
  kind: FighterKind,
  pos: Point,
}

#[derive(Copy, Clone)]
enum Cell {
  Wall,
  Vacant { previous: Point, dist: u16 },
  Occupied { kind: FighterKind, id: u8 },
}

impl Cell {
  const MAX: u16 = u16::MAX;

  fn is_vacant(&self) -> bool {
    match self {
      Cell::Vacant { .. } => true,
      _ => false,
    }
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // https://stackoverflow.com/q/41390457/183120
    let symbol = match self {
      Cell::Wall => b'#',
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
  fn idx_to_point(&self, idx: usize) -> Point {
    Point((idx / self.width) as u8, (idx % self.width) as u8)
  }

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
        let neighbours = [
          Point(fighter.pos.0, fighter.pos.1 - 1),
          Point(fighter.pos.0 - 1, fighter.pos.1),
          Point(fighter.pos.0 + 1, fighter.pos.1),
          Point(fighter.pos.0, fighter.pos.1 + 1),
        ];
        let mut targets = Vec::<Point>::with_capacity(4);
        for n in neighbours {
          if self.is_vacant(n) {
            targets.push(n);
          }
        }
        targets
      })
      .collect()
  }

  // reset distances stored in vacant cells to default
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

  fn next_step(&mut self, src: Point, dsts: &Vec<Point>) -> Option<Point> {
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
    let mut found_dst: Option<Point> = None;

    'outer: while !to_visit.is_empty() {
      mem::swap(&mut visiting, &mut to_visit);
      to_visit.clear();
      cur_dist += 1;
      while let Some((pt, from)) = visiting.pop() {
        let cell = self.cell(pt); // proceed only if cell is vacant
        if let Some(Cell::Vacant { previous, dist }) = cell {
          if dist > cur_dist {
            self.set(
              pt,
              Cell::Vacant {
                previous: from,
                dist: cur_dist,
              },
            );
            if let Some(found_idx) = dsts.iter().position(|&p| p == pt) {
              // |cur_dist| ascends so closest distance has been found; break
              found_dst = Some(dsts[found_idx]);
              break 'outer;
            }
            to_visit.push((Point(pt.0, pt.1 + 1), pt));
            to_visit.push((Point(pt.0 + 1, pt.1), pt));
            to_visit.push((Point(pt.0 - 1, pt.1), pt));
            to_visit.push((Point(pt.0, pt.1 - 1), pt));
          }
        }
      }
    }

    // if found a path, backtrack and choose the optimal next step
    match found_dst {
      Some(mut pt) => {
        loop {
          if let Some(Cell::Vacant { previous, .. }) = self.cell(pt) {
            match self.is_vacant(previous) {
              true => pt = previous,
              false => break,
            }
          } else {
            panic!("Unexpected state")
          }
        }
        Some(pt)
      }
      None => None,
    }
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
      .map(|(idx, symbol)| {
        let id = fighters.len() as u8;
        match symbol {
          b'#' => Cell::Wall,
          b'E' => {
            fighters.push(Fighter {
              id,
              kind: FighterKind::Elf,
              pos: Point(idx as u8, height as u8),
            });
            Cell::Occupied {
              kind: FighterKind::Elf,
              id,
            }
          }
          b'G' => {
            fighters.push(Fighter {
              id,
              kind: FighterKind::Goblin,
              pos: Point(idx as u8, height as u8),
            });
            Cell::Occupied {
              kind: FighterKind::Goblin,
              id,
            }
          }
          _ => Cell::Vacant {
            previous: Point::default(),
            dist: Cell::MAX,
          },
        }
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
  println!("{}", map);

  let test_pt = Point(1, 1);
  let targets = map.targets(test_pt);
  println!("{:?}", targets);
  if let Some(pt) = map.next_step(test_pt, &targets) {
    println!("{:?}", pt);
  }

  Ok(())
}
