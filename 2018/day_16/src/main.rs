use std::{
  collections::{HashMap, HashSet},
  error::Error,
  io::{self, BufRead},
};

struct Cpu<'a> {
  reg: [u8; 4],
  // operations jump table
  ops: [Jump<'a>; 16],
}

#[derive(Clone, Copy)]
struct Jump<'a> {
  opcode: i8,
  name: &'a str,
  fnptr: fn(&mut Cpu<'a>, u8, u8, u8),
}

impl Cpu<'_> {
  fn new() -> Self {
    Cpu {
      reg: [0, 0, 0, 0],
      ops: [
        Jump {
          opcode: -1,
          name: &"addr",
          fnptr: Self::addr,
        },
        Jump {
          opcode: -1,
          name: &"addi",
          fnptr: Self::addi,
        },
        Jump {
          opcode: -1,
          name: &"mulr",
          fnptr: Self::mulr,
        },
        Jump {
          opcode: -1,
          name: &"muli",
          fnptr: Self::muli,
        },
        Jump {
          opcode: -1,
          name: &"banr",
          fnptr: Self::banr,
        },
        Jump {
          opcode: -1,
          name: &"bani",
          fnptr: Self::bani,
        },
        Jump {
          opcode: -1,
          name: &"borr",
          fnptr: Self::borr,
        },
        Jump {
          opcode: -1,
          name: &"bori",
          fnptr: Self::bori,
        },
        Jump {
          opcode: -1,
          name: &"setr",
          fnptr: Self::setr,
        },
        Jump {
          opcode: -1,
          name: &"seti",
          fnptr: Self::seti,
        },
        Jump {
          opcode: -1,
          name: &"gtir",
          fnptr: Self::gtir,
        },
        Jump {
          opcode: -1,
          name: &"gtri",
          fnptr: Self::gtri,
        },
        Jump {
          opcode: -1,
          name: &"gtrr",
          fnptr: Self::gtrr,
        },
        Jump {
          opcode: -1,
          name: &"eqir",
          fnptr: Self::eqir,
        },
        Jump {
          opcode: -1,
          name: &"eqri",
          fnptr: Self::eqri,
        },
        Jump {
          opcode: -1,
          name: &"eqrr",
          fnptr: Self::eqrr,
        },
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
    self.reg[c as usize] = a;
  }

  fn gtir(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (a > self.reg[b as usize]) as u8;
  }

  fn gtri(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (self.reg[a as usize] > b) as u8;
  }

  fn gtrr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (self.reg[a as usize] > self.reg[b as usize]) as u8;
  }

  fn eqir(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (a == self.reg[b as usize]) as u8;
  }

  fn eqri(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (self.reg[a as usize] == b) as u8;
  }

  fn eqrr(&mut self, a: u8, b: u8, c: u8) {
    self.reg[c as usize] = (self.reg[a as usize] == self.reg[b as usize]) as u8;
  }
}

#[derive(Default, Debug, Copy, Clone)]
struct Sample {
  pre: [u8; 4],
  instr: [u8; 4],
  post: [u8; 4],
}

fn possible_opcodes<'a>(
  mut cpu: &mut Cpu<'a>,
  map: &mut HashMap<&'a str, HashSet<u8>>,
  sample: Sample,
) -> usize {
  let ops = cpu.ops.clone();
  ops
    .iter()
    .filter(|&op| {
      cpu.reg = sample.pre;
      (op.fnptr)(&mut cpu, sample.instr[1], sample.instr[2], sample.instr[3]);
      cpu.reg == sample.post
    })
    .fold(0usize, |acc, op| {
      let s = map.entry(op.name).or_insert(HashSet::new());
      s.insert(sample.instr[0]);
      acc + 1
    })
}

/** Reads a string of delimiter-seperated list of 4 values into `reg`
   e.g. `"14, 0, 2, 1"`
*/
fn load_values(list: &str, delim: &str, reg: &mut [u8; 4]) {
  let tokens = list
    .split(delim)
    .filter_map(|s| s.parse::<u8>().ok())
    .collect::<Vec<u8>>();
  reg[..4].clone_from_slice(&tokens[..4]);
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

  let mut cpu = Cpu::new();
  let mut map = HashMap::<&str, HashSet<u8>>::with_capacity(16);
  let count_exceeding_3 = samples
    .iter()
    .filter(|&&sample| possible_opcodes(&mut cpu, &mut map, sample) > 2)
    .count();
  println!("Samples similar to 3+ opcodes: {}", count_exceeding_3);

  Ok(())
}
