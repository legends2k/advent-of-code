use std::{
  collections::HashMap,
  error::Error,
  io::{self, BufRead},
};

type Word = u32;
type InputWord = u8;
type Instruction = [InputWord; 4];

struct Cpu<'a> {
  reg: [Word; Cpu::REG_COUNT as usize],
  ip: u8, // < REG_COUNT
  op:
    [fn(&mut Cpu<'a>, InputWord, InputWord, InputWord); Cpu::OP_COUNT as usize],
}

impl Cpu<'_> {
  const REG_COUNT: u8 = 6;
  const OP_COUNT: u8 = 16;

  /** Create a CPU with IP bound to a register; `ip_reg` is unchecked */
  fn new(ip_reg: u8) -> Self {
    Cpu {
      reg: [0; Self::REG_COUNT as usize],
      ip: ip_reg,
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

  fn run(&mut self, program: &[Instruction]) {
    self.set_ip(0);
    while self.get_ip() < program.len() {
      let i = program[self.get_ip()];
      (self.op[i[0] as usize])(self, i[1], i[2], i[3]);
      self.inc_ip();
    }
  }

  fn inc_ip(&mut self) {
    self.reg[self.ip as usize] += 1;
  }

  fn set_ip(&mut self, value: Word) {
    self.reg[self.ip as usize] = value;
  }

  fn get_ip(&mut self) -> usize {
    self.reg[self.ip as usize] as usize
  }

  // operations
  fn addr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] + self.reg[b as usize];
  }

  fn addi(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] + b as Word;
  }

  fn mulr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] * self.reg[b as usize];
  }

  fn muli(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] * b as Word;
  }

  fn banr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] & self.reg[b as usize];
  }

  fn bani(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] & b as Word;
  }

  fn borr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] | self.reg[b as usize];
  }

  fn bori(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize] | b as Word;
  }

  fn setr(&mut self, a: InputWord, _: InputWord, c: InputWord) {
    self.reg[c as usize] = self.reg[a as usize];
  }

  fn seti(&mut self, a: InputWord, _: InputWord, c: InputWord) {
    self.reg[c as usize] = a as Word;
  }

  fn gtir(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = (a as Word > self.reg[b as usize]) as Word;
  }

  fn gtri(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = (self.reg[a as usize] > b as Word) as Word;
  }

  fn gtrr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] =
      (self.reg[a as usize] > self.reg[b as usize]) as Word;
  }

  fn eqir(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = (a as Word == self.reg[b as usize]) as Word;
  }

  fn eqri(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] = (self.reg[a as usize] == b as Word) as Word;
  }

  fn eqrr(&mut self, a: InputWord, b: InputWord, c: InputWord) {
    self.reg[c as usize] =
      (self.reg[a as usize] == self.reg[b as usize]) as Word;
  }
}

fn parse_program() -> Result<Vec<Instruction>, Box<dyn Error>> {
  let mut mnemonics_to_opcodes = HashMap::with_capacity(16);
  mnemonics_to_opcodes.insert("borr", 0u8);
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
      let value = match t.parse::<InputWord>() {
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
  if reg_id >= Cpu::REG_COUNT {
    eprintln!(
      "Invalid input: specify a register in [0, {}) range.",
      Cpu::REG_COUNT
    );
    return Ok(());
  }
  let program = parse_program()?;
  let mut cpu = Cpu::new(reg_id);

  cpu.run(&program);
  println!("Value of register 0: {}", cpu.reg[0]);

  Ok(())
}
