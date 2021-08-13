use std::error::Error;
use std::io::{self, BufRead};

struct Cpu {
  reg: [u8; 4],
  // operations
  ops: [(String, fn(&mut Cpu, u8, u8, u8)); 16],
}

impl Cpu {
  fn new() -> Self {
    Cpu {
      reg: [0, 0, 0, 0],
      ops: [
        ("addr".to_string(), Self::addr),
        ("addi".to_string(), Self::addi),
        ("mulr".to_string(), Self::mulr),
        ("muli".to_string(), Self::muli),
        ("banr".to_string(), Self::muli),
        ("bani".to_string(), Self::muli),
        ("borr".to_string(), Self::muli),
        ("bori".to_string(), Self::muli),
        ("setr".to_string(), Self::muli),
        ("seti".to_string(), Self::muli),
        ("gtir".to_string(), Self::muli),
        ("gtri".to_string(), Self::muli),
        ("gtrr".to_string(), Self::muli),
        ("eqir".to_string(), Self::muli),
        ("eqri".to_string(), Self::muli),
        ("eqrr".to_string(), Self::muli),
      ],
    }
  }

  // operations
  fn addr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] + self.reg[b as usize];
  }

  fn addi(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] + b;
  }

  fn mulr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] * self.reg[b as usize];
  }

  fn muli(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] * b;
  }

  fn banr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] & self.reg[b as usize];
  }

  fn bani(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] & b;
  }

  fn borr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] | self.reg[b as usize];
  }

  fn bori(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize] | b;
  }

  fn setr(&mut self, a: u8, _: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize];
  }

  fn seti(&mut self, a: u8, _: u8, c: u8) {
    self.reg[c as usize] = self.reg[a as usize];
  }

  fn gtir(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match a > self.reg[b as usize] {
      true => 1,
      false => 0,
    };
  }

  fn gtri(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match self.reg[a as usize] > b {
      true => 1,
      false => 0,
    };
  }

  fn gtrr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match self.reg[a as usize] > self.reg[b as usize] {
      true => 1,
      false => 0,
    };
  }

  fn eqir(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match a == self.reg[b as usize] {
      true => 1,
      false => 0,
    };
  }

  fn eqri(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match self.reg[a as usize] == b {
      true => 1,
      false => 0,
    };
  }

  fn eqrr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = match self.reg[a as usize] == self.reg[b as usize] {
      true => 1,
      false => 0,
    };
  }
}

#[derive(Default, Debug, Copy, Clone)]
struct Sample {
  pre: [u8; 4],
  instr: [u8; 4],
  post: [u8; 4],
}

fn possible_opcodes(mut cpu: &mut Cpu, s: Sample) -> usize {
  let ops = cpu.ops.clone();
  ops
    .iter()
    .filter(|(_, op)| {
      cpu.reg = s.pre;
      op(&mut cpu, s.instr[1], s.instr[2], s.instr[3]);
      cpu.reg == s.post
    })
    .count()
}

/** Reads a string of delimiter-seperated list of 4 values into `reg`
   e.g. `"14, 0, 2, 1"`
*/
fn load_values(list: &str, delim: &str, reg: &mut [u8; 4]) {
  let tokens = list
    .split(delim)
    .filter_map(|s| s.parse::<u8>().ok())
    .collect::<Vec<u8>>();
  for i in 0..4 {
    reg[i] = tokens[i];
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  // parse input
  let mut samples = Vec::<Sample>::with_capacity(900);
  let mut sample_program = Vec::<[u8; 4]>::with_capacity(900);
  let mut current_sample: Option<Sample> = None;
  for l in io::stdin().lock().lines() {
    let line = l?;
    if !line.is_empty() {
      match (current_sample, line.bytes().next()) {
        (Some(mut sample), Some(b'A')) => {
          load_values(&line[9..19], ", ", &mut sample.post);
          samples.push(sample);
          current_sample = None;
        }
        (Some(mut sample), _) => {
          load_values(&line, " ", &mut sample.instr);
          current_sample = Some(sample);
        }
        (None, Some(b'B')) => {
          let mut sample = Sample::default();
          load_values(&line[9..19], ", ", &mut sample.pre);
          current_sample = Some(sample);
        }
        (None, _) => {
          // part 2 input
          let mut values = [0u8; 4];
          load_values(&line, " ", &mut values);
          sample_program.push(values);
        }
      }
    }
  }

  let mut c = Cpu::new();
  Ok(())
}
