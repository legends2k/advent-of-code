use std::{
  fmt::{self, Debug, Formatter},
  io::{self, BufRead, Error},
  iter, mem,
};

/** Map with 2 extra rows and cols surrounding. */
#[derive(Clone)]
struct Map {
  data: Vec<u8>,
  cols: u16,
  rows: u16,
}

impl Map {
  fn to_idx(&self, row: u16, col: u16) -> usize {
    (row * (self.cols + 2) + col) as usize
  }

  /** Iterates over adjacent cells and shortcircuits if f returns true */
  fn check_adjs<P>(&self, idx: usize, f: P) -> bool
  where
    P: Fn(u8, u8) -> bool,
  {
    let row0 = idx - 1 - (self.cols as usize + 2);
    let row1 = idx - 1;
    let row2 = idx - 1 + (self.cols as usize + 2);
    let adj_idxs = [
      row0,
      row0 + 1,
      row0 + 2,
      row1,
      row1 + 2,
      row2,
      row2 + 1,
      row2 + 2,
    ];
    // For a 10 × 10 input, the adj indices of 0 × 0 cell would be
    // | 0   1   2|  3  4  5  6  7  8  9 10 11
    // |12 (13) 14| 15 16 17 18 19 20 21 22 23
    // |24  25  26| 27 ...
    let (mut trees, mut lumberyard) = (0, 0);
    adj_idxs.iter().for_each(|&i| match self.data[i] {
      b'.' | 0 => (),
      b'|' => trees += 1,
      b'#' => lumberyard += 1,
      _ => unreachable!(),
    });
    f(trees, lumberyard)
  }

  fn flip(&self, other: &mut Self) {
    for j in 1..=self.rows {
      for i in 1..=self.cols {
        let idx = self.to_idx(j, i);
        let image = match self.data[idx] {
          b'.' => match self.check_adjs(idx, |trees, _| trees >= 3) {
            true => b'|',
            false => b'.',
          },
          b'|' => match self.check_adjs(idx, |_, lumberyard| lumberyard >= 3) {
            true => b'#',
            false => b'|',
          },
          b'#' => {
            match self.check_adjs(idx, |trees, lumberyard| {
              lumberyard >= 1 && trees >= 1
            }) {
              true => b'#',
              false => b'.',
            }
          }
          _ => unreachable!(),
        };
        other.data[idx] = image;
      }
    }
  }

  fn value(&self) -> u32 {
    let (woods, lumberyards) =
      self.data.iter().fold((0, 0), |totals, &c| match c {
        b'|' => (totals.0 + 1, totals.1),
        b'#' => (totals.0, totals.1 + 1),
        _ => totals,
      });
    woods * lumberyards
  }
}

impl Debug for Map {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for j in 1..=self.rows {
      for i in 1..=self.cols {
        write!(f, "{}", self.data[self.to_idx(j, i)] as char)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

fn main() -> Result<(), Error> {
  let mut line = String::new();
  let size = match io::stdin().read_line(&mut line) {
    Ok(n) => n - 1, // sub ‘\n’
    Err(_) => panic!("Invalid input"),
  };
  let mut m1 = Map {
    data: Vec::<u8>::with_capacity((size + 2) * (size + 2)),
    rows: size as u16,
    cols: size as u16,
  };
  m1.data.extend(iter::repeat(0).take(size + 2));
  m1.data.push(0);
  m1.data.extend_from_slice(line[0..size].as_bytes());
  m1.data.push(0);

  for l in io::stdin().lock().lines() {
    let line = l?;
    m1.data.push(0);
    m1.data.extend_from_slice(line[0..size].as_bytes());
    m1.data.push(0);
  }
  m1.data.extend(iter::repeat(0).take(size + 2));

  let mut m2 = m1.clone();
  for _ in 0..10 {
    mem::swap(&mut m1.data, &mut m2.data);
    m1.flip(&mut m2);
  }
  // println!("{:?}", m2);

  // Part 1
  println!(
    "Total resource value of lumber after 10 mins: {}",
    m2.value()
  );

  Ok(())
}
