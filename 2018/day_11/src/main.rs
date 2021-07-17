use std::env;
use std::process;

const SIZE: usize = 300;

fn main() {
  let serial: usize = env::args()
    .nth(1)
    .unwrap_or_else(|| {
      eprintln!("Usage: day_11 SERIAL");
      process::exit(1);
    })
    .parse()
    .unwrap_or_else(|err| {
      eprintln!("Failed to convert SERIAL into a number: {}", err);
      process::exit(1);
    });

  // compute cell powers
  let mut cells = [0i32; SIZE * SIZE];
  for row in 0..SIZE {
    for col in 0..SIZE {
      let rack = col + 11;
      let t = (rack * (row + 1) + serial) * rack;
      cells[(row * SIZE + col) as usize] = ((t as i32 / 100) % 10) - 5;
    }
  }

  // Part 1: 3×3 square in cells with maximum power
  let (idx, pow) = max_square(&cells, 3);
  {
    let (row, col) = idx_to_2d(idx);
    println!(
      "Maximum 3x3 square power: {} at {},{}",
      pow,
      col + 1,
      row + 1
    );
  }

  // Part 2: k×k square in cells with maximum power
  {
    let mut max = -std::i32::MAX;
    let mut max_idx: usize = 0;
    let mut square = 4;
    for i in 4..SIZE as usize {
      let (idx, local_max) = max_square(&cells, i);
      if max < local_max {
        max = local_max;
        max_idx = idx;
        square = i;
      }
    }
    let (row, col) = idx_to_2d(max_idx);
    println!(
      "Maximum power: {} at {},{},{}",
      max,
      col + 1,
      row + 1,
      square
    );
  }
}

// Returns |dim|-sized submatrix’s left-top with maximum power from |cells|
fn max_square(cells: &[i32; SIZE * SIZE], dim: usize) -> (usize, i32) {
  let mut cells_n = *cells;

  // Overall approach is brute force but in a cache-friendly way.  We calculate
  // sum of every |dim|×|dim| square possible in |cells| and return the maximum.
  // However, we don’t compute each square’s value locally (per-cell).  Instead
  // for each row, sum |dim|-sized lists horizontally upto the last eligible
  // column.  In the resulting grid, each eligible element (where a |dim|×|dim|
  // square is within bounds) will have its horizontal |dim|-sized list’s sum.
  // Now do the same vertically; eligible elements in resulting grid will have
  // sum of all elements in |dim|×|dim| square starting there.  Data access
  // pattern is either sequential or sequantial with constant offset.

  // Perhaps the most optimised solution might be using a _summed area table_
  // https://blog.demofox.org/2018/04/16/prefix-sums-and-summed-area-tables/

  // Horizontal summing
  // NOTE: ALL rows are eligible, while only some columns are
  for row in 0..SIZE {
    for col in 0..(SIZE - (dim - 1)) {
      let idx = idx_from_2d(row, col);
      cells_n[idx] += cells_n.iter().skip(idx + 1).take(dim - 1).sum::<i32>();
    }
  }

  // Vertical summing
  // Compute only for eligible elements where |dim|-sized squares are possible
  for col in 0..(SIZE - (dim - 1)) {
    for row in 0..(SIZE - (dim - 1)) {
      let idx = idx_from_2d(row, col);
      cells_n[idx] += cells_n
        .iter()
        .skip(idx + SIZE)
        .step_by(SIZE)
        .take(dim - 1)
        .sum::<i32>();
    }
  }

  // Find maximum element from eligible set; left-top of maximum powered square
  let (idx, &power) = cells_n
    .iter()
    .enumerate()
    // shouldn’t use ‘take_while’ as it’d stop yeilding after first ‘false’
    .filter(|(i, _)| {
      let (row, col) = idx_to_2d(*i);
      ((col + dim) <= SIZE) && ((row + dim) <= SIZE)
    })
    .max_by_key(|(_, &x)| x)
    .expect("No maximum for given dimension!");
  (idx, power)
}

fn idx_from_2d(row: usize, col: usize) -> usize {
  row * SIZE + col
}

fn idx_to_2d(idx: usize) -> (usize, usize) {
  let row = idx / SIZE;
  (row, idx - row * SIZE)
}

// convenience debug function; unused
#[allow(dead_code)]
fn print_matrix(cells: &[i32; (SIZE * SIZE)]) {
  for i in 0..SIZE {
    for j in 0..SIZE {
      print!("{:4}", cells[idx_from_2d(i, j)]);
    }
    println!("");
  }
}
