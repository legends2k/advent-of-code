use core::str::FromStr;
use std::{
  error::Error,
  fmt::{self, Debug, Formatter},
  io::{self, BufRead},
  ops::{Add, AddAssign, Rem, Sub},
  thread::sleep,
  time::Duration,
};

#[derive(Debug, Default, Copy, Clone)]
struct Point(i32, i32);

impl Add<i32> for Point {
  type Output = Self;

  fn add(self, delta: i32) -> Self {
    Point(self.0 + delta, self.1)
  }
}

impl Sub<i32> for Point {
  type Output = Self;

  fn sub(self, delta: i32) -> Self {
    Point(self.0 - delta, self.1)
  }
}

impl AddAssign<i32> for Point {
  /** Get next or previous point */
  fn add_assign(&mut self, delta: i32) {
    self.0 += delta;
  }
}

impl Rem<i32> for Point {
  type Output = Self;

  fn rem(self, delta: i32) -> Self {
    Point(self.0, self.1 + delta)
  }
}

#[derive(Debug, Default)]
struct Line {
  end: [Point; 2],
}

impl Line {
  /** Return Line with varying X, but stable Y */
  fn new_dx(fixed: i32, min: i32, max: i32) -> Self {
    Line {
      end: [Point(min, fixed), Point(max, fixed)],
    }
  }

  /** Return Line with varying Y, but stable X */
  fn new_dy(fixed: i32, min: i32, max: i32) -> Self {
    Line {
      end: [Point(fixed, min), Point(fixed, max)],
    }
  }

  fn offset_by(&self, p: Point) -> Self {
    Line {
      end: [
        Point(self.end[0].0 - p.0, self.end[0].1 - p.1),
        Point(self.end[1].0 - p.0, self.end[1].1 - p.1),
      ],
    }
  }

  fn is_horizontal(&self) -> bool {
    self.end[0].1 == self.end[1].1
  }

  /** Sort end points to have reading order */
  fn normalize(&mut self) {
    match (
      self.is_horizontal(),
      (self.end[0].0 > self.end[1].0),
      (self.end[0].1 > self.end[1].1),
    ) {
      (true, true, _) | (false, _, true) => self.end.swap(0, 1),
      _ => (),
    };
  }
}

impl FromStr for Line {
  type Err = Box<dyn Error>;

  fn from_str(input: &str) -> Result<Line, Self::Err> {
    let parts = input.split(", ").collect::<Vec<_>>();
    let fixed = parts[0][2..].parse::<i32>()?;
    let range = parts[1][2..].split("..").collect::<Vec<_>>();
    let min = range[0].parse::<i32>()?;
    let max = range[1].parse::<i32>()?;
    match parts[0].as_bytes()[0] {
      b'x' => Ok(Line::new_dy(fixed, min, max)),
      b'y' => Ok(Line::new_dx(fixed, min, max)),
      _ => Err(Box::<dyn Error>::from("Invalid input line")),
    }
  }
}

struct Ground {
  cols: usize,
  rows: usize,
  data: Vec<u8>,
}

impl Debug for Ground {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "\x1B[2J\x1B[1;1H")?;
    for r in 0..self.rows {
      for c in 0..self.cols {
        write!(
          f,
          "{}",
          char::from(self.data[self.to_idx(Point(c as i32, r as i32))])
        )?;
      }
      writeln!(f)?;
    }
    sleep(Duration::from_secs_f32(0.75));
    Ok(())
  }
}

impl Ground {
  fn to_idx(&self, pt: Point) -> usize {
    pt.1 as usize * self.cols + pt.0 as usize
  }

  fn set(&mut self, ch: u8, l: Line) {
    match l.is_horizontal() {
      true => {
        let idx = self.to_idx(l.end[0]);
        self.data[idx..=(idx + (l.end[1].0 - l.end[0].0) as usize)].fill(ch);
      }
      false => {
        let offset = self.to_idx(l.end[0]);
        self.data[offset..]
          .iter_mut()
          .step_by(self.cols)
          .take((l.end[1].1 - l.end[0].1 + 1) as usize)
          .for_each(|c| *c = ch);
      }
    }
  }

  fn set_point(&mut self, p: Point, ch: u8) {
    let idx = self.to_idx(p);
    self.data[idx] = ch;
  }

  fn get_point(&self, p: Point) -> u8 {
    let idx = self.to_idx(p);
    self.data[idx]
  }

  fn spring_streams(&mut self, origin: Point, streams: &mut Vec<Stream>) {
    let sides = [origin - 1, origin + 1];
    let states = [State::Left(origin), State::Right(origin)];
    for (i, &s) in sides.iter().enumerate() {
      if self.get_point(s) == b'.' {
        self.set_point(s, b'!');
        streams.push(Stream {
          state: states[i],
          pos: s,
        });
      }
    }
  }

  /**
    Given one wall of a pot, find other.  Requires wet (‘|’) intervening blocks
    and blocking underlying blocks (wall or water; ‘#’ or ‘~’). `dir` should be
    `-1` for searching left.
  */
  fn opposite_wall(&self, mut p: Point, dir: i32) -> Option<Point> {
    let mut below = p % 1;
    while (self.get_point(p) == b'|')
      && ((self.get_point(below) == b'#') || (self.get_point(below) == b'~'))
    {
      p += dir;
      below = p % 1;
    }
    match (self.get_point(p), self.get_point(below)) {
      (b'#', b'#') | (b'#', b'~') => Some(p - dir),
      _ => None,
    }
  }
}

#[derive(Copy, Clone)]
enum State {
  Down,         // flowing down
  Wall,         // resting; offsprings at work
  Left(Point),  // at exit check if bound by ‘#’ both sides and fill
  Right(Point), // --- do ---
}

struct Stream {
  state: State,
  pos: Point,
}

impl Stream {
  fn set_position(&mut self, p: Point, g: &mut Ground) {
    self.pos = p;
    g.set_point(p, b'!');
  }

  fn flow(&mut self, g: &mut Ground) -> Vec<Self> {
    let mut new_streams = Vec::with_capacity(2);
    let below = self.pos % 1;
    match self.state {
      State::Wall => (),
      State::Down => {
        g.set_point(self.pos, b'|');
        match g.get_point(below) {
          // TODO: add ‘|’ and ‘!’?
          b'.' => self.set_position(below, g),
          b'#' | b'~' => {
            self.state = State::Wall;
            g.spring_streams(self.pos, &mut new_streams);
          }
          _ => unreachable!(),
        }
      }
      State::Left(origin) | State::Right(origin) => {
        g.set_point(self.pos, b'|');
        match g.get_point(below) {
          b'.' => {
            self.set_position(below, g);
            self.state = State::Down;
          }
          _ => {
            let (side, opp_dir) = match self.state {
              State::Left(_) => (self.pos - 1, 1),
              State::Right(_) => (self.pos + 1, -1),
              _ => unreachable!(),
            };
            match g.get_point(side) {
              b'.' => self.set_position(side, g),
              b'#' => {
                // stop and/or fill + create offspring
                if let Some(wall) = g.opposite_wall(self.pos, opp_dir) {
                  let mut water = Line::new_dx(self.pos.1, self.pos.0, wall.0);
                  water.normalize();
                  g.set(b'~', water);
                  g.spring_streams(origin % -1, &mut new_streams);
                }
                self.state = State::Wall;
              }
              _ => unreachable!(),
            }
          }
        }
      }
    }
    new_streams
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  // Find X interval; expand by 1 to accommodate overflow beyond farthest pot.
  // Find Y-max; grow by 1 avoiding out of bounds checks. Y-min is always 1.
  let (mut min, mut max) = (Point(i32::MAX, 1), Point(i32::MIN, i32::MIN));
  let mut lines = Vec::<Line>::with_capacity(1700);
  for l in io::stdin().lock().lines() {
    let l = Line::from_str(&l?)?;
    min.0 = min.0.min(l.end[0].0.min(l.end[1].0));
    max.0 = max.0.max(l.end[0].0.max(l.end[1].0));
    max.1 = max.1.max(l.end[0].1.max(l.end[1].1));
    lines.push(l);
  }
  min.0 -= 1;
  max.0 += 1;

  println!("Min: {:?}, Max: {:?}", min, max);
  let rows = (max.1 - min.1 + 1) as usize;
  let cols = (max.0 - min.0 + 1) as usize;
  println!("Ground size: {} × {}\n", cols, rows);
  let mut ground = Ground {
    cols,
    rows,
    data: vec![b'.'; rows * cols],
  };

  // plot the scan
  for line in &lines {
    let l = line.offset_by(min);
    ground.set(b'#', l);
  }
  // set first stream
  let eternal_spring = Point(500 - min.0, 0);
  ground.set_point(eternal_spring, b'!');
  let mut streams = Vec::with_capacity(10);
  streams.push(Stream {
    state: State::Down,
    pos: eternal_spring,
  });
  println!("{:?}", ground);

  let mut new_streams = Vec::with_capacity(4);
  for _ in 0..45 {
    for s in streams.iter_mut() {
      new_streams.append(&mut s.flow(&mut ground));
    }
    println!("{:?}", ground);
    streams.append(&mut new_streams);
  }

  Ok(())
}
