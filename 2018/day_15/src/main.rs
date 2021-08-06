use core::fmt;
use std::{
  cmp::Ordering,
  collections::HashMap,
  error::Error,
  fmt::Display,
  io::{stdin, BufRead},
  mem,
  num::NonZeroU8,
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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FighterKind {
  Elf(NonZeroU8),
  Goblin(NonZeroU8),
}

impl FighterKind {
  const INITIAL_ATTACK: u8 = 3;

  fn new_elf(attack: u8) -> Self {
    FighterKind::Elf(NonZeroU8::new(attack).unwrap())
  }

  fn new_goblin(attack: u8) -> Self {
    FighterKind::Goblin(NonZeroU8::new(attack).unwrap())
  }

  fn attacks(&self) -> u8 {
    match self {
      FighterKind::Elf(attack) | FighterKind::Goblin(attack) => attack.get(),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fighter {
  kind: FighterKind,
  pos: Point,
  hits: u8,
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
  const INITIAL_HITS: u8 = 200;

  fn is_alive(&self) -> bool {
    self.hits > 0
  }

  /** Returns next attacker ID if available */
  fn target(&self, map: &Map) -> Option<u8> {
    let mut enemies: Vec<u8> = [
      Point(self.pos.0, self.pos.1 - 1),
      Point(self.pos.0 - 1, self.pos.1),
      Point(self.pos.0 + 1, self.pos.1),
      Point(self.pos.0, self.pos.1 + 1),
    ]
    .iter()
    .filter(|&&pt| {
      matches!(map.cell(pt),
               Some(Cell::Occupied { kind, .. }) if kind != self.kind)
    })
    .map(|&pt| map.layout[map.point_to_idx(pt)].get_fighter_id())
    .collect();
    // Fighter’s PartialOrd sorts only by |pos|; we want by |hits| first
    enemies.sort_unstable_by_key(|idx| {
      (map.fighters[idx].hits, map.fighters[idx].pos)
    });
    enemies.first().copied()
  }
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
    matches!(self, Cell::Vacant { .. })
  }

  fn get_fighter_id(&self) -> u8 {
    match self {
      Cell::Occupied { id, .. } => *id,
      _ => unreachable!(),
      // not going with panic! as never called on non-Occupied call
    }
  }

  fn get_kind(&self) -> FighterKind {
    match self {
      Cell::Occupied { kind, .. } => *kind,
      _ => unreachable!(),
      // not going with panic! as never called on non-Occupied call
    }
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
        kind: FighterKind::Elf(_),
        ..
      } => b'E',
      Cell::Occupied {
        kind: FighterKind::Goblin(_),
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
  fighters: HashMap<u8, Fighter>,
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
    matches!(self.cell(pt), Some(cell) if cell.is_vacant())
  }

  fn targets(&self, from: Point) -> Vec<Point> {
    let cell = &self.layout[self.point_to_idx(from)];
    self
      .fighters
      .iter()
      .filter(|&(_, fighter)| {
        fighter.is_alive() && fighter.kind != cell.get_kind()
      })
      .flat_map(|(_, fighter)| {
        [
          Point(fighter.pos.0, fighter.pos.1 - 1),
          Point(fighter.pos.0 - 1, fighter.pos.1),
          Point(fighter.pos.0 + 1, fighter.pos.1),
          Point(fighter.pos.0, fighter.pos.1 + 1),
        ]
        .iter()
        .filter(|&&pt| self.is_vacant(pt))
        .copied()
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
          match self.layout[self.point_to_idx(pt)] {
            Cell::Vacant { previous, .. } => match self.is_vacant(previous) {
              true => pt = previous,
              false => break,
            },
            _ => unreachable!(),
          }
        }
        Some(pt)
      }
      None => None,
    }
  }

  fn move_fighter(&mut self, idx: &u8, pt: Point) {
    let old_idx = self.point_to_idx(self.fighters[idx].pos);
    self.layout[old_idx] = Cell::Vacant {
      previous: Point::default(),
      dist: Cell::MAX,
    };
    // editing a value in a mutable hash map
    // https://stackoverflow.com/a/30414450/183120
    self.fighters.get_mut(idx).unwrap().pos = pt;
    let new_idx = self.point_to_idx(pt);
    self.layout[new_idx] = Cell::Occupied {
      kind: self.fighters[idx].kind,
      id: *idx,
    };
  }

  /** Attack `unit`.  Return true if `unit` is dead after attack */
  fn attack(&mut self, unit: u8, attacks: u8) -> bool {
    self.fighters.get_mut(&unit).unwrap().hits =
      self.fighters[&unit].hits.saturating_sub(attacks);
    let attacked = &self.fighters[&unit];
    if attacked.hits == 0 {
      let idx = self.point_to_idx(attacked.pos);
      self.layout[idx] = Cell::Vacant {
        previous: Point::default(),
        dist: Cell::MAX,
      };
    }
    attacked.hits == 0
  }

  /** Set new attack points for Elves */
  fn set_elves_attack(&mut self, new_attack: u8) {
    // can’t mutate self.layout while mutating self.fighters; note and do later
    // https://stackoverflow.com/a/45724688/183120
    let mut to_update = Vec::new();
    for fighter in self.fighters.values_mut() {
      if matches!(fighter.kind, FighterKind::Elf(_)) {
        fighter.kind = FighterKind::new_elf(new_attack);
        to_update.push(fighter.pos);
      }
    }
    for pt in to_update {
      let idx = self.point_to_idx(pt);
      let elf_id = self.layout[idx].get_fighter_id();
      self.layout[idx] = Cell::Occupied {
        kind: FighterKind::new_elf(new_attack),
        id: elf_id,
      };
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

struct RoundsAndHits(u32, u32);

fn battle(map: &mut Map, no_elf_dies: bool) -> Option<RoundsAndHits> {
  let mut fighter_ids = map.fighters.keys().copied().collect::<Vec<_>>();
  let mut rounds = 0u32;
  let mut victory = false;
  'battle: while !victory {
    // fix turn order amongst fighters
    fighter_ids.sort_unstable_by_key(|idx| map.fighters[idx].pos);
    for idx in &fighter_ids {
      // let dead warriors rest in peace
      if map.fighters[idx].hits != 0 {
        // A turn is not just a move or an attack but it can be move + attack
        // when the move positions fighter next (“adjacent”) to an enemy.
        // Move
        if map.fighters[idx].target(&map).is_none() {
          let targets = map.targets(map.fighters[idx].pos);
          match (map.next_step(map.fighters[idx].pos, &targets), victory) {
            (Some(pt), _) => map.move_fighter(idx, pt),
            // battle ceased in the middle of a round, don’t increment |rounds|
            (None, true) => break 'battle,
            (None, false) => (), // skip turn and lay in wait
          }
        }
        // Attack
        if let Some(enemy) = map.fighters[idx].target(&map) {
          if map.attack(enemy, map.fighters[idx].kind.attacks()) {
            // enemy dead after attack; mark victory if no enemies are left
            let enemy_kind = map.fighters[&enemy].kind;
            if no_elf_dies && matches!(enemy_kind, FighterKind::Elf(_)) {
              return None;
            }
            victory = !map
              .fighters
              .values()
              .any(|f| f.is_alive() && f.kind == enemy_kind);
          }
        }
      }
    }
    rounds += 1;
  }
  let hits_left = map.fighters.values().map(|f| f.hits as u32).sum::<u32>();
  Some(RoundsAndHits(rounds, hits_left))
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut layout =
    Vec::<Cell>::with_capacity(MAP_DIMENSION_MAX * MAP_DIMENSION_MAX);
  let mut fighters = HashMap::<u8, Fighter>::with_capacity(32);

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
            fighters.insert(
              id,
              Fighter {
                kind: FighterKind::new_elf(FighterKind::INITIAL_ATTACK),
                pos: Point(idx as u8, height as u8),
                hits: Fighter::INITIAL_HITS,
              },
            );
            Cell::Occupied {
              kind: FighterKind::new_elf(FighterKind::INITIAL_ATTACK),
              id,
            }
          }
          b'G' => {
            fighters.insert(
              id,
              Fighter {
                kind: FighterKind::new_goblin(FighterKind::INITIAL_ATTACK),
                pos: Point(idx as u8, height as u8),
                hits: Fighter::INITIAL_HITS,
              },
            );
            Cell::Occupied {
              kind: FighterKind::new_goblin(FighterKind::INITIAL_ATTACK),
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

  // part 1
  {
    let mut map = Map {
      layout: layout.clone(),
      fighters: fighters.clone(),
      width,
      height,
    };
    let RoundsAndHits(rounds, hits_left) =
      battle(&mut map, /*no_elf_dies*/ false).unwrap();
    println!(
      "Outcome: {} (rounds) × {} (hit points): {}",
      rounds,
      hits_left,
      rounds * hits_left
    );
  }

  // part 2
  let mut elf_attack = 4u8;
  loop {
    let mut map = Map {
      layout: layout.clone(),
      fighters: fighters.clone(),
      width,
      height,
    };
    map.set_elves_attack(elf_attack);
    if let Some(RoundsAndHits(rounds, hits_left)) =
      battle(&mut map, /*no_elf_dies*/ true)
    {
      println!(
        "Outcome: {} (rounds) × {} (hit points): {}",
        rounds,
        hits_left,
        rounds * hits_left
      );
      println!("With {} attacks no elves die!", elf_attack);
      break;
    }
    elf_attack += 1;
  }

  Ok(())
}
