use std::collections::VecDeque;
use std::fmt;
use std::io::{self, BufRead, Error};
use std::str::FromStr;

#[derive(Clone)]
struct Plants {
  pot: VecDeque<bool>,
  start_idx: i32,
}

impl Plants {
  fn is_pot_planted(&self, idx: i32) -> bool {
    let i = idx - self.start_idx;
    match (i >= 0) && (i < self.pot.len() as i32) {
      true => self.pot[i as usize],
      false => false,
    }
  }

  // Return |idx|-th pot’s configuration as an unsigned value where
  // last five bits represent pot with its left and right neighbours
  fn pot_configuration(&self, idx: i32) -> u8 {
    ((idx - 2)..=(idx + 2))
      .into_iter()
      .enumerate()
      .map(|(idx, pot_idx)| (self.is_pot_planted(pot_idx) as u8) << (4 - idx))
      // print binary with zero padding; 7 includes ‘0b’ prefix
      // https://stackoverflow.com/a/44690529/183120
      // .inspect(|x| println!("{:#07b}", x))
      .fold(0, |acc, x| acc | x)
  }

  fn set(&mut self, idx: i32, value: bool) {
    let mut adjusted_idx = idx - self.start_idx;
    if adjusted_idx < 0 {
      for _ in adjusted_idx..0 {
        self.pot.push_front(false);
      }
      self.start_idx = idx;
      adjusted_idx = 0;
    } else if adjusted_idx >= self.pot.len() as i32 {
      self.pot.resize(1 + adjusted_idx as usize, false);
    }
    self.pot[adjusted_idx as usize] = value;
  }

  fn front(&self) -> i32 {
    self.start_idx
  }

  fn back(&self) -> i32 {
    self.start_idx + self.pot.len() as i32 - 1
  }

  fn trim(&mut self) {
    while *self.pot.back().expect("Last can't be empty") == false {
      self.pot.pop_back();
    }
    while *self.pot.front().expect("First can't be empty") == false {
      self.pot.pop_front();
      self.start_idx += 1;
    }
  }

  fn sum_planted_pot_id(&self) -> i32 {
    self
      .pot
      .iter()
      .enumerate()
      .filter(|(_, &value)| value)
      .map(|(idx, _)| self.start_idx + idx as i32)
      .sum()
  }
}

impl FromStr for Plants {
  type Err = Error;
  fn from_str(input: &str) -> Result<Plants, Self::Err> {
    const PREFIX: &str = "initial state: ";
    Ok(Plants {
      pot: input.chars().skip(PREFIX.len()).map(|c| c == '#').collect(),
      start_idx: 0,
    })
  }
}

impl fmt::Debug for Plants {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let indices: String = (self.start_idx
      ..(self.start_idx + self.pot.len() as i32))
      .map(|i| format!("{:3}", i))
      .collect();
    let values: String = self
      .pot
      .iter()
      .map(|x| {
        format!(
          "  {}",
          match x {
            true => '#',
            false => '.',
          }
        )
      })
      .collect();
    write!(f, "{}\n{}", indices, values)
  }
}

fn generations(plants: &Plants, rules: &[bool; 32], iterations: u64) {
  let mut cur_gen = plants.clone();
  let mut next_gen = cur_gen.clone();
  // println!("Pots now\n{:?}", plants);
  for _ in 1..=iterations {
    // start checking for germination from current interval ± 3 pots
    for i in (cur_gen.front() - 3)..=(cur_gen.back() + 3) {
      next_gen.set(i, rules[cur_gen.pot_configuration(i) as usize]);
    }
    // remove needless elements due to previous set()s
    next_gen.trim();
    // avoid needless allocation; reuse same objects with their internals
    std::mem::swap(&mut cur_gen, &mut next_gen);
  }
  // println!("Pots after {} generations\n{:?}", iterations, cur_gen);
  println!(
    "Sum of planted pot IDs after {} generations: {}",
    iterations,
    cur_gen.sum_planted_pot_id()
  );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut init_state = String::new();
  io::stdin()
    .read_line(&mut init_state)
    .expect("Expected initial state!");
  let plants: Plants =
    init_state.parse().expect("Unable to parse initial state");

  let mut rules = [false; 32];
  for l in io::stdin().lock().lines() {
    let line = l?;
    if line.ends_with("#") {
      let idx: u8 = line[0..5]
        .chars()
        .enumerate()
        .map(|(idx, flag)| ((flag == '#') as u8) << (4 - idx))
        .fold(0, |acc, x| acc | x);
      rules[idx as usize] = true;
      // println!("{} --> rule[{}] = {}", line, idx, rules[idx as usize]);
    }
  }
  assert_eq!(
    rules[0b00000] || rules[0b00001] || rules[0b10000],
    false,
    "Implementation won't work for this input"
  );

  // Part 1: germinate pots for 20 generations
  generations(&plants, &rules, 20);

  Ok(())
}
