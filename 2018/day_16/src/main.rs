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

struct Sample {
  pre: [u8; 4],
  instr: [u8; 4],
  post: [u8; 4],
}

fn possible_ops(mut cpu: &mut Cpu, s: Sample) -> usize {
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

fn main() {
  let mut c = Cpu::new();
}
