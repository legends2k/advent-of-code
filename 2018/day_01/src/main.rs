use std::io::{self, BufRead, Error};
use std::result::Result;

fn main() {
  let result_freq: i32 = io::stdin()
    .lock()
    .lines()
    .map(|line: Result<String, Error>| -> i32 {
      line.unwrap_or_default().parse::<i32>().unwrap_or_default()
    })
    .collect::<Vec<_>>()
    .iter()
    .sum();
  println!("{}", result_freq);
}
