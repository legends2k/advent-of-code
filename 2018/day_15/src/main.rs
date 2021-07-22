use core::fmt;
use std::{
  error::Error,
  fmt::Display,
  io::{stdin, stdout, BufRead, Write},
};

#[derive(Copy, Clone, Debug)]
struct Point(u8, u8);

#[derive(Debug, PartialEq, Eq)]
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

enum Cell {
  Wall,
  Vacant(u16),
  Occupied { kind: FighterKind, id: u8 },
}

impl Cell {
  fn is_vacant(&self) -> bool {
    match self {
      Cell::Vacant(_) => true,
      _ => false,
    }
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // https://stackoverflow.com/q/41390457/183120
    let symbol = match self {
      Cell::Wall => b'#',
      Cell::Vacant(_) => b'.',
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

  fn cell(&self, pt: Point) -> &Cell {
    &self.layout[self.point_to_idx(pt)]
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
          if self.cell(n).is_vacant() {
            targets.push(n);
          }
        }
        targets
      })
      .collect()
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
          _ => Cell::Vacant(std::u16::MAX),
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

  Ok(())
}
