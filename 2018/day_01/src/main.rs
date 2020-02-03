use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let freq: Vec<_> = stdin
    .lock()
    .lines()
    .map(|line| -> i32 {
      line.unwrap_or_default().parse::<i32>().unwrap_or_default()
    })
    .collect();
  println!("Total {} lines", freq.len());
}
