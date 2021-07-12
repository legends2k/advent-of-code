use std::env;
use std::process;

fn main() {
  let serial: i32 = env::args()
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

  const SIZE: i32 = 300;
  let mut cells = [0; (SIZE * SIZE) as usize];
  for row in 0..SIZE {
    for col in 0..SIZE {
      let rack = col + 11;
      let t = (rack * (row + 1) + serial) * rack;
      cells[(row * SIZE + col) as usize] = ((t / 100) % 10) - 5;
    }
  }

  // compute cumulative powers in cache-friendly way
  // compute cumulative X powers
  for row in 0..(SIZE - 2) {
    for col in 0..(SIZE - 2) {
      let idx = (row * SIZE + col) as usize;
      cells[idx] += cells[idx + 1] + cells[idx + 2];
    }
  }

  // compute cumulative Y powers
  for col in 0..(SIZE - 2) {
    for row in 0..(SIZE - 2) {
      let idx = (row * SIZE + col) as usize;
      cells[idx] += cells[idx + SIZE as usize] + cells[idx + SIZE as usize * 2];
    }
  }

  if let Some((i, _)) = cells.iter().enumerate().max_by_key(|(_, &x)| x) {
    let row = i as i32 / SIZE;
    let col = i as i32 - row * SIZE;
    println!("Maximum powered square at {},{}", col + 1, row + 1);
  }
}
