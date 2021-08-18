use std::{
  error::Error,
  fmt::{self, Debug, Formatter},
  io::{self, BufRead},
};

struct Cpu<'a> {
  reg: [u16; 4],
  // operations jump table
  ops: [Jump<'a>; 16],
}

#[derive(Clone, Copy)]
struct Jump<'a> {
  opcode: i8,
  name: &'a str,
  fnptr: fn(&mut Cpu<'a>, u16, u16, u16),
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

  fn clear(&mut self) {
    self.reg.iter_mut().for_each(|r| *r = 0);
  }

  fn execute(&mut self, instr: &[u16; 4]) {
    (self.ops[instr[0] as usize].fnptr)(self, instr[1], instr[2], instr[3]);
  }

  // operations
  fn addr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] + self.reg[b as usize];
  }

  fn addi(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] + b;
  }

  fn mulr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] * self.reg[b as usize];
  }

  fn muli(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] * b;
  }

  fn banr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] & self.reg[b as usize];
  }

  fn bani(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] & b;
  }

  fn borr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] | self.reg[b as usize];
  }

  fn bori(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize] | b;
  }

  fn setr(&mut self, a: u16, _: u16, c: u16) {
    self.reg[c as usize] = self.reg[a as usize];
  }

  fn seti(&mut self, a: u16, _: u16, c: u16) {
    self.reg[c as usize] = a;
  }

  fn gtir(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = (a > self.reg[b as usize]) as u16;
  }

  fn gtri(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = (self.reg[a as usize] > b) as u16;
  }

  fn gtrr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = (self.reg[a as usize] > self.reg[b as usize]) as u16;
  }

  fn eqir(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = (a == self.reg[b as usize]) as u16;
  }

  fn eqri(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] = (self.reg[a as usize] == b) as u16;
  }

  fn eqrr(&mut self, a: u16, b: u16, c: u16) {
    self.reg[c as usize] =
      (self.reg[a as usize] == self.reg[b as usize]) as u16;
  }
}

// https://stackoverflow.com/a/38150040/183120
#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_print {
  ($( $args:expr ),*) => {};
}

impl Debug for Jump<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{} --> {}", self.opcode, self.name)
  }
}

#[derive(Default, Debug, Copy, Clone)]
struct Sample {
  pre: [u16; 4],
  instr: [u16; 4],
  post: [u16; 4],
}

fn possible_opcodes<'a>(
  mut cpu: &mut Cpu<'a>,
  opcode_to_fnptr: &mut [u16; 16],
  sample: Sample,
) -> usize {
  let ops = cpu.ops;
  ops
    .iter()
    .enumerate()
    .filter(|(_, &op)| {
      cpu.reg = sample.pre;
      (op.fnptr)(&mut cpu, sample.instr[1], sample.instr[2], sample.instr[3]);
      cpu.reg == sample.post
    })
    .fold(0, |acc, (idx, _)| {
      opcode_to_fnptr[sample.instr[0] as usize] |= 1u16 << idx;
      acc + 1
    })
}

/** Reads a string of delimiter-seperated list of 4 values into `reg`
   e.g. `"14, 0, 2, 1"`
*/
fn load_values(list: &str, delim: &str, reg: &mut [u16; 4]) {
  let tokens = list
    .split(delim)
    .filter_map(|s| s.parse::<u16>().ok())
    .collect::<Vec<u16>>();
  reg.copy_from_slice(&tokens);
}

fn main() -> Result<(), Box<dyn Error>> {
  // parse input
  let mut samples = Vec::<Sample>::with_capacity(900);
  let mut program = Vec::<[u16; 4]>::with_capacity(900);
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
          let mut values = [0; 4];
          load_values(&line, " ", &mut values);
          program.push(values);
        }
      }
    }
  }

  // part 1
  let mut cpu = Cpu::new();
  let mut opcode_to_fnptr = [0; 16];
  let count_exceeding_3 = samples
    .iter()
    .filter(|&&sample| {
      possible_opcodes(&mut cpu, &mut opcode_to_fnptr, sample) > 2
    })
    .count();
  println!("Samples similar to 3+ opcodes: {}", count_exceeding_3);

  // part 2.1: resolve opcodes and mnemonics
  while let Some(opcode) = opcode_to_fnptr
    .iter()
    .position(|fnptrs| fnptrs.count_ones() == 1)
  {
    let fnptr_idx = opcode_to_fnptr[opcode].trailing_zeros();
    cpu.ops[fnptr_idx as usize].opcode = opcode as i8;
    debug_print!("{:?}", cpu.ops[fnptr_idx as usize]);
    let mask = !opcode_to_fnptr[opcode];
    opcode_to_fnptr
      .iter_mut()
      .for_each(|fnptrs| *fnptrs &= mask);
  }
  if cpu.ops.iter().any(|j| j.opcode == -1) {
    return Err(Box::<dyn Error>::from(
      "Insufficient data: can't resolve opcodes from sample set",
    ));
  } else {
    // sort |Cpu::ops| to align array index to opcode
    cpu.ops.sort_unstable_by(|a, b| a.opcode.cmp(&b.opcode));
  }

  // part 2.2: run program
  cpu.clear();
  program.iter().for_each(|stmt| cpu.execute(stmt));
  println!("Register 0: {}", cpu.reg[0]);

  Ok(())
}
