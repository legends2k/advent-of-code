use std::io::{self, BufRead};
use std::ops::{Fn, RangeBounds, RangeInclusive};
use std::str::FromStr;

struct Board([[u8; 1000]; 1000]);

impl Board {
  fn new() -> Self {
    Board([[0_u8; 1000]; 1000])
  }

  fn intensity(&self) -> u64 {
    self.0.iter().fold(0_u64, |s, r: &[u8; 1000]| {
      s + r.iter().fold(0_u64, |s, r| *r as u64 + s)
    })
  }

  // accept both Range and RangeInclusive with RangeBounds
  fn operate<F, R>(&mut self, f: F, rows: R, cols: R)
  where
    F: Fn(u8) -> u8,
    R: RangeBounds<usize> + Iterator<Item = usize> + Clone,
  {
    for col in cols {
      // clone to avoid consuming rows
      for row in rows.clone() {
        self.0[row][col] = f(self.0[row][col]);
      }
    }
  }
}

fn to_ranges(ranges: [&str; 2]) -> [RangeInclusive<usize>; 2] {
  let mut stops = [0_usize; 4];
  for (i, v) in ranges
    .iter()
    .flat_map(|s| s.splitn(2, ','))
    .map(|si| usize::from_str(si).ok())
    .filter_map(|s| s)
    .enumerate()
  {
    stops[i] = v;
  }
  [
    RangeInclusive::new(stops[0], stops[2]),
    RangeInclusive::new(stops[1], stops[3]),
  ]
}

fn main() {
  let mut b1 = Board::new();
  let mut b2 = Board::new();

  let ops1 = [|_| 1, |_| 0, |v| v ^ 1];
  let ops2 = [|v: u8| v + 1, |v: u8| v.saturating_sub(1), |v: u8| v + 2];

  for l in io::stdin().lock().lines() {
    if let Ok(s) = l {
      let toks: Vec<_> = s.split_ascii_whitespace().collect();
      match (toks[0], toks[1]) {
        ("turn", "on") => {
          let [r1, r2] = to_ranges([toks[2], toks[4]]);
          b1.operate(ops1[0], r1.clone(), r2.clone());
          b2.operate(ops2[0], r1, r2);
        }
        ("turn", "off") => {
          let [r1, r2] = to_ranges([toks[2], toks[4]]);
          b1.operate(ops1[1], r1.clone(), r2.clone());
          b2.operate(ops2[1], r1, r2);
        }
        ("toggle", _) => {
          let [r1, r2] = to_ranges([toks[1], toks[3]]);
          b1.operate(ops1[2], r1.clone(), r2.clone());
          b2.operate(ops2[2], r1, r2)
        }
        _ => unreachable!(),
      }
    }
  }

  println!("{}", b1.intensity());
  println!("{}", b2.intensity());
}
