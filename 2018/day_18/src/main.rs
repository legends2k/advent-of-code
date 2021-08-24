use std::{
  fmt::{self, Debug, Formatter},
  io::{self, BufRead, Error},
  mem,
};

/** Map with 2 extra rows and cols surrounding; implementation detail. */
#[derive(Clone)]
struct Map {
  data: Vec<u8>,
  cols: u16,
  rows: u16,
}

impl Map {
  fn new(size: usize) -> Self {
    Map {
      data: vec![0; (size + 2) * (size + 2)],
      cols: size as u16,
      rows: size as u16,
    }
  }

  /** Set `data` from `values`; assumes input of length: `cols` and indices
   *  starting from 0, ignoring the dummy boundary rows and cols */
  fn set(&mut self, row: usize, values: &[u8]) {
    let start = self.to_idx(row as u16 + 1, 1);
    self.data[start..(start + self.cols as usize)].copy_from_slice(values);
  }

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
      b'|' => trees += 1,
      b'#' => lumberyard += 1,
      _ => (),
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

/** Checks if `values` has a repeating sequence at the end;
returns its length if found */
fn has_repeating_sequence(values: &[u32]) -> Option<usize> {
  let value = match values.last() {
    Some(&x) => x,
    None => return None,
  };
  if let Some(f) = values.iter().rev().skip(2).position(|&v| v == value) {
    // Current value was generated earlier; check if there’s a
    // repeating sequence placed back to back
    let border = values.len() - 2 - f;
    let expected_len = values.len() - border;
    // check if it’s repeating ignoring small sequences
    if (expected_len > 2)
      && (border >= expected_len)
      && (values[(border - expected_len)..border] == values[border..])
    {
      return Some(expected_len);
    }
  }
  None
}

/** Runs `iterations` of simulation but try short-circuiting after
`short` times */
fn simulate(m: &Map, iterations: u32, short: u16) -> u32 {
  let (mut m1, mut m2) = (m.clone(), m.clone());

  let mut values = Vec::with_capacity(1024);
  for i in 0..iterations {
    mem::swap(&mut m1.data, &mut m2.data);
    m1.flip(&mut m2);

    if i >= short.into() {
      let this_value = m2.value();
      values.push(this_value);
      if let Some(seq_len) = has_repeating_sequence(&values) {
        // println!("Found {}-value repeating sequence at {}", seq_len, i);
        // values
        //   .iter()
        //   .rev()
        //   .take(seq_len)
        //   .rev()
        //   .for_each(|&x| print!("{}, ", x));
        let pending = (iterations - 1 - i) as usize;
        // `- 1` because `pending` is count while we want index
        let idx_into_seq = (pending - 1) % seq_len;
        let base = values.len() - seq_len;
        return values[base + idx_into_seq];
      }
    }
  }
  // println!("{:?}", m2);
  m2.value()
}

fn main() -> Result<(), Error> {
  let mut line = String::new();
  let size = match io::stdin().read_line(&mut line) {
    Ok(n) => n - 1, // sub ‘\n’
    Err(_) => panic!("Invalid input"),
  };
  let mut m = Map::new(size);
  let mut row = 0;
  m.set(row, line[0..size].as_bytes());
  for l in io::stdin().lock().lines() {
    let line = l?;
    row += 1;
    m.set(row, line[0..size].as_bytes());
  }

  // Part 1
  println!(
    "Total resource value of lumber after 10 mins: {}",
    simulate(&m, 10, 10)
  );

  // Part 2
  println!(
    "Total resource value of lumber after 1,000,000,000 mins: {}",
    simulate(&m, 1_000_000_000, 500)
  );

  Ok(())
}
