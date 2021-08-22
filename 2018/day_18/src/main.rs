/** Map with 2 extra rows and cols surrounding but `cols` and `rows`
   are the actual values from input
*/
struct Map {
  data: Vec<u8>,
  cols: u16,
  rows: u16,
}

impl Map {
  fn to_idx(&self, row: u16, col: u16) -> usize {
    (row * self.cols + col) as usize
  }

  /** Iterates over adjacent cells and shortcircuits if f returns true */
  fn check<P>(&self, idx: usize, f: P) -> bool
  where
    P: Fn(u8, u8, u8) -> bool,
  {
    let row0 = idx - self.cols as usize - 1;
    let row1 = idx - 1;
    let row2 = idx + self.cols as usize - 1;
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
    let (mut ground, mut trees, mut lumberyard) = (0, 0, 0);
    adj_idxs.iter().for_each(|&i| match self.data[i] {
      b'.' => ground += 1,
      b'|' => trees += 1,
      b'#' => lumberyard += 1,
      _ => unreachable!(),
    });
    f(ground, trees, lumberyard)
  }

  fn flip(&self, other: &mut Self) {
    for j in 1..=self.rows {
      for i in 1..=self.cols {
        let idx = self.to_idx(j, i);
        let image = match self.data[idx] {
          b'.' => match self.check(idx, |_, trees, _| trees >= 3) {
            true => b'|',
            false => b'.',
          },
          b'|' => match self.check(idx, |_, _, lumberyard| lumberyard >= 3) {
            true => b'#',
            false => b'|',
          },
          b'#' => {
            match self
              .check(idx, |_, trees, lumberyard| lumberyard >= 1 && trees >= 1)
            {
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
}

fn main() {}
