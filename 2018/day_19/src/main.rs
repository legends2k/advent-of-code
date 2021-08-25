use std::{
  collections::HashMap,
  error::Error,
  io::{self, BufRead},
};

struct Cpu<'a> {
  reg: [u16; 6],
  ip: u8,
  op: [fn(&mut Cpu<'a>, u16, u16, u16); 16],
}

impl Cpu<'_> {
  fn new(ip: u8) -> Self {
    Cpu {
      reg: [0; 6],
      ip,
      op: [
        Self::borr,
        Self::addr,
        Self::eqrr,
        Self::addi,
        Self::eqri,
        Self::eqir,
        Self::gtri,
        Self::mulr,
        Self::setr,
        Self::gtir,
        Self::muli,
        Self::banr,
        Self::seti,
        Self::gtrr,
        Self::bani,
        Self::bori,
      ],
    }
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

type Instruction = [u16; 4];

fn parse_program() -> Result<Vec<Instruction>, Box<dyn Error>> {
  let mut mnemonics_to_opcodes = HashMap::<&str, u16>::with_capacity(16);
  mnemonics_to_opcodes.insert("borr", 0);
  mnemonics_to_opcodes.insert("addr", 1);
  mnemonics_to_opcodes.insert("eqrr", 2);
  mnemonics_to_opcodes.insert("addi", 3);
  mnemonics_to_opcodes.insert("eqri", 4);
  mnemonics_to_opcodes.insert("eqir", 5);
  mnemonics_to_opcodes.insert("gtri", 6);
  mnemonics_to_opcodes.insert("mulr", 7);
  mnemonics_to_opcodes.insert("setr", 8);
  mnemonics_to_opcodes.insert("gtir", 9);
  mnemonics_to_opcodes.insert("muli", 10);
  mnemonics_to_opcodes.insert("banr", 11);
  mnemonics_to_opcodes.insert("seti", 12);
  mnemonics_to_opcodes.insert("gtrr", 13);
  mnemonics_to_opcodes.insert("bani", 14);
  mnemonics_to_opcodes.insert("bori", 15);

  let mut program = Vec::with_capacity(64);
  for (i, l) in io::stdin().lock().lines().enumerate() {
    let line = l?;
    let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
    if tokens.len() != 4 {
      return Err(Box::<dyn Error>::from(
        "Invalid input: unexpected number of tokens in instruction",
      ));
    }
    let mut instr: Instruction = [0; 4];
    instr[0] = match mnemonics_to_opcodes.get(tokens[0]) {
      Some(&code) => code,
      None => {
        return Err(Box::<dyn Error>::from("Unrecognized operation name"))
      }
    };
    // following can’t be done in a functional fashion since iter::map’s closure
    // can’t do returning of a Result from this function
    for (idx, &t) in tokens[1..4].iter().enumerate() {
      let value = match t.parse::<u16>() {
        Ok(i) => i,
        Err(_) => {
          return Err(Box::<dyn Error>::from(format!(
            "Invalid argument to instruction at line {}",
            i + 1
          )))
        }
      };
      instr[idx + 1] = value;
    }
    program.push(instr);
  }
  Ok(program)
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut line = String::new();
  io::stdin().read_line(&mut line)?;
  if !line.starts_with("#ip") {
    eprintln!("Invalid input: expected macro binding IP to a register.");
    return Ok(());
  }
  line.pop(); // drop LF
  let reg_id = line
    .as_bytes()
    .last()
    .map(|&reg_id| reg_id.wrapping_sub(b'0'))
    .ok_or("Invalid input: unable to parse register specification.")?;
  if reg_id > 5 {
    eprintln!("Invalid input: specify a register in [0, 5] range.");
    return Ok(());
  }
  let pgm = parse_program()?;

  Ok(())
}
