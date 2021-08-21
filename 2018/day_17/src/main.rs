use core::str::FromStr;
use std::{
  error::Error,
  fmt::{self, Debug, Formatter},
  fs::File,
  io::{self, BufRead, Write},
  ops::{Add, AddAssign, Rem, Sub},
  thread::sleep,
  time::Duration,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
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
  cols: i32,
  rows: i32,
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
    sleep(Duration::from_secs_f32(0.25));
    Ok(())
  }
}

impl Ground {
  fn to_idx(&self, pt: Point) -> usize {
    (pt.1 * self.cols + pt.0) as usize
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
          .step_by(self.cols as usize)
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

  fn find_ground(&self, mut p: Point) -> Point {
    let c = self.get_point(p);
    // keep skipping until we reach a different block like ‘#’, ‘~’
    // or ‘|’ in case we reach another stream’s ebb out
    while p.1 < self.rows && self.get_point(p) == c {
      p = p % 1;
    }
    p % -1
  }

  /**
    Given one wall of a pot, find other.  Requires wet (‘|’) intervening blocks
    and blocking underlying blocks (wall or water; ‘#’ or ‘~’). `dir` should be
    `-1` for searching left.  Returns (_, false) if it’s not a barrier plane.
  */
  fn opposite_wall(&self, mut p: Point, dir: i32) -> (Point, bool) {
    let mut below = p % 1;
    while (self.get_point(p) != b'#')  // skip ‘.’ and ‘|’
      && ((self.get_point(below) == b'#') || (self.get_point(below) == b'~'))
    {
      p += dir;
      below = p % 1;
    }
    match (self.get_point(p), self.get_point(below)) {
      (b'#', b'#') | (b'#', b'~') => ((p - dir), true),
      _ => (p, false),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
  Down,
  Fill,
  Wait(u8), // state while children are at work; may/may not resume
  Done,
  Gone,
}

struct Stream {
  state: State,
  to_fill: u16,
  pos: Point,
  parent: i32,
}

impl Stream {
  fn new(pos: Point, parent: i32) -> Self {
    Stream {
      state: State::Down,
      pos,
      to_fill: 0,
      parent,
    }
  }

  fn is_alive(&self) -> bool {
    match self.state {
      State::Down | State::Fill => true,
      State::Done | State::Wait(_) | State::Gone => false,
    }
  }

  fn flow(&mut self, idx: usize, g: &mut Ground, new_streams: &mut Vec<Self>) {
    match self.state {
      State::Down => {
        let bottom = g.find_ground(self.pos % 1);
        g.set(b'|', Line::new_dy(self.pos.0, self.pos.1, bottom.1));
        let distance = bottom.1 - self.pos.1;
        self.pos = bottom;
        self.state = match (bottom.1 + 1) < g.rows {
          true => match g.get_point(self.pos % 1) {
            b'|' => State::Gone,
            _ => {
              self.to_fill = distance as u16;
              State::Fill
            }
          },
          false => State::Gone, // reached end of input
        };
      }
      State::Fill => {
        let (left, wall_l) = g.opposite_wall(self.pos, -1);
        let (right, wall_r) = g.opposite_wall(self.pos, 1);
        match (wall_l, wall_r) {
          (true, true) => {
            g.set(b'~', Line::new_dx(self.pos.1, left.0, right.0));
            self.pos = self.pos % -1;
            self.to_fill -= 1;
            // done with stream; unblock parent stream
            if self.to_fill == 0 {
              self.state = State::Done;
            }
          }
          (true, false) => {
            g.set(b'|', Line::new_dx(self.pos.1, left.0, right.0));
            new_streams.push(Stream::new(right, idx as i32));
            self.state = State::Wait(1);
          }
          (false, true) => {
            g.set(b'|', Line::new_dx(self.pos.1, left.0, right.0));
            new_streams.push(Stream::new(left, idx as i32));
            self.state = State::Wait(1);
          }
          (false, false) => {
            // both arms beget children
            g.set(b'|', Line::new_dx(self.pos.1, left.0, right.0));
            new_streams.push(Stream::new(left, idx as i32));
            new_streams.push(Stream::new(right, idx as i32));
            self.state = State::Wait(2);
          }
        }
      }
      _ => (),
    }
  }
}

fn log_to_file(ground: &Ground) -> Result<(), Box<dyn Error>> {
  let mut o = File::create("output")?;
  for j in 0..ground.rows {
    for i in 0..ground.cols {
      let idx = (j * ground.cols + i) as usize;
      write!(o, "{}", ground.data[idx] as char)?;
    }
    writeln!(o)?;
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  // Find X interval; expand by 1 to accommodate overflow beyond farthest pot.
  // Find Y-max; grow by 1 avoiding out of bounds checks. Y-min is always 1.
  let (mut min, mut max) =
    (Point(i32::MAX, i32::MAX), Point(i32::MIN, i32::MIN));
  let mut lines = Vec::<Line>::with_capacity(1700);
  for l in io::stdin().lock().lines() {
    let l = Line::from_str(&l?)?;
    min.0 = min.0.min(l.end[0].0.min(l.end[1].0));
    min.1 = min.1.min(l.end[0].1.min(l.end[1].1));
    max.0 = max.0.max(l.end[0].0.max(l.end[1].0));
    max.1 = max.1.max(l.end[0].1.max(l.end[1].1));
    lines.push(l);
  }
  min.0 -= 1;
  max.0 += 1;

  let rows = max.1 - min.1 + 1;
  let cols = max.0 - min.0 + 1;
  let mut ground = Ground {
    cols,
    rows,
    data: vec![b'.'; (rows * cols) as usize],
  };

  // plot the scan
  for line in &lines {
    let l = line.offset_by(min);
    ground.set(b'#', l);
  }
  // set first stream
  let eternal_spring = Point(500 - min.0, 0);
  let mut streams = Vec::with_capacity(2_00_000);
  streams.push(Stream::new(eternal_spring, -1));
  ground.set_point(eternal_spring, b'|');

  let mut new_streams = Vec::with_capacity(32);
  while streams.iter().any(|s| s.is_alive()) {
    let n = streams.len();
    for idx in 0..n {
      if streams[idx].state != State::Done {
        streams[idx].flow(idx, &mut ground, &mut new_streams);
        if streams[idx].state == State::Done && streams[idx].parent >= 0 {
          let parent_id = streams[idx].parent as usize;
          streams[parent_id].state = match streams[parent_id].state {
            State::Wait(child) if child == 1 => State::Fill,
            State::Wait(child) => State::Wait(child - 1),
            _ => streams[parent_id].state,
          };
        }
      }
    }
    streams.append(&mut new_streams);
  }

  println!(
    "Count of moist tiles: {}",
    ground
      .data
      .iter()
      .filter(|&&c| c == b'|' || c == b'~')
      .count()
  );
  println!(
    "Count of water tiles: {}",
    ground.data.iter().filter(|&&c| c == b'~').count()
  );

  // log_to_file(&ground)?;

  Ok(())
}
