// https://adventofcode.com/2018/day/1

use std::collections::BTreeSet;
use std::io::{self, BufRead, Error};
use std::result::Result;

fn main() {
  let freq: Vec<i32> = io::stdin()
    .lock()
    .lines()
    .map(|line: Result<String, Error>| -> i32 {
      line.unwrap_or_default().parse::<i32>().unwrap_or_default()
    })
    .collect::<Vec<_>>();

  // part 1
  let result_freq: i32 = freq.iter().sum();
  println!("{}", result_freq);

  // part 2
  let mut resulting_freqs = BTreeSet::new();
  resulting_freqs.insert(0); // 0 is the frequency device starts with
  let mut interrim_freq = 0;
  for f in freq.iter().cycle() {
    interrim_freq += f;
    if !resulting_freqs.insert(interrim_freq) {
      break;
    }
  }
  println!("First repeating frequency: {}", interrim_freq);
}
