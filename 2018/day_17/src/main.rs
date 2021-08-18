use core::str::FromStr;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead};

#[derive(Debug, Default, Copy, Clone)]
struct Point(i32, i32);

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

impl Ground {
  fn to_idx(&self, pt: Point) -> usize {
    pt.1 as usize * self.cols + pt.0 as usize
  }

  fn set(&mut self, ch: u8, p1: Point, p2: Point) {
    match p1.1 == p2.1 {
      true => {
        let idx = self.to_idx(p1);
        self.data[idx..=(idx + (p2.0 - p1.0) as usize)].fill(ch);
      }
      false => {
        let offset = self.to_idx(p1);
        self.data[offset..]
          .iter_mut()
          .step_by(self.cols)
          .take((p2.1 - p1.1 + 1) as usize)
          .for_each(|c| *c = ch);
      }
    }
  }
}

impl Debug for Ground {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    Ok(())
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  // find X interval; expand by 1 to accommodate overflow beyond farthest pot
  // find Y-max; Y-min is always 1
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
    ground.set(b'#', l.end[0], l.end[1]);
  }
  println!("{:?}", ground);

  Ok(())
}
