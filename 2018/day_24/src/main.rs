use num_traits::PrimInt;
use pest::Parser;
use pest_derive::Parser;
use std::{
  collections::HashMap,
  error::Error,
  fmt,
  io::{self, Read},
  str::FromStr,
};

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;

struct Attacks(u8);

impl Attacks {
  fn is(&self, other: &Attacks) -> bool {
    (other.0 & self.0) != 0
  }
}

impl fmt::Debug for Attacks {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "0b{:b}", self.0)
  }
}

struct Group {
  units: u32,
  hits: u32,
  damages: u16,
  id: u16,
  attack: Attacks,
  initiative: u8,
  immunity: Attacks,
  weakness: Attacks,
}

impl Group {
  fn effective_power(&self) -> u32 {
    self.units * self.damages as u32
  }

  fn is_alive(&self) -> bool {
    self.units > 0
  }
}

#[derive(Default)]
struct Army<'a> {
  groups: Vec<Group>,
  name: &'a str,
}

// PrimInt is yet to get the BITS member; make a new trait.
// https://stackoverflow.com/q/73711297/183120
trait Bits {
  const BITS: usize;
}
macro_rules! impl_bits {
  ( $($ty:ident)* ) => {
    $(
      impl Bits for $ty {
        const BITS: usize = Self::BITS as usize;
      }
    )*
  };
}
impl_bits!(u8 u16 u32 u64 u128);

fn to_flag<'a, T: Bits + PrimInt>(
  attack: &'a str,
  attack_to_flag: &mut HashMap<&'a str, T>,
) -> Result<T, Box<dyn Error>> {
  let n = attack_to_flag.len();
  let mask = T::one() << n;
  match n < T::BITS {
    true => Ok(*attack_to_flag.entry(attack).or_insert(mask)),
    false => Err(Box::<dyn Error>::from(
      "More than {T::BITS} distinct attacks; insufficient bit-width.",
    )),
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut input_str = String::new();
  let mut stdin = io::stdin();
  stdin.read_to_string(&mut input_str)?;
  let input = InputParser::parse(Rule::file, &input_str)
    .expect("Invalid input")
    .next()
    .unwrap();

  let mut armies = [Army::default(), Army::default()];
  let mut next_army: u8 = 0;
  let mut next_group: u16 = 0;
  let mut attack_to_flag: HashMap<&str, u8> = HashMap::new();
  for line in input.into_inner() {
    match line.as_rule() {
      Rule::army_name => {
        armies[next_army as usize].name = line.as_str();
        next_army += 1;
        next_group = 0;
      }
      Rule::group => {
        let mut counts = [0u32; 4];
        let mut idx = 0;
        let mut attack = Attacks(0);
        let mut immunities = 0u8;
        let mut weaknesses = 0u8;
        for r in line.into_inner() {
          match r.as_rule() {
            Rule::count => {
              counts[idx] = u32::from_str(r.as_str())?;
              idx += 1;
            }
            Rule::attack => {
              attack = Attacks(to_flag(r.as_str(), &mut attack_to_flag)?);
            }
            Rule::traits => {
              for t in r.into_inner() {
                match t.as_rule() {
                  Rule::immunities => {
                    for i in t.into_inner() {
                      immunities |= to_flag(i.as_str(), &mut attack_to_flag)?;
                    }
                  }
                  Rule::weaknesses => {
                    for w in t.into_inner() {
                      weaknesses |= to_flag(w.as_str(), &mut attack_to_flag)?;
                    }
                  }
                  _ => unreachable!(),
                }
              }
            }
            _ => unreachable!(),
          }
        }
        armies[(next_army - 1) as usize].groups.push(Group {
          units: counts[0],
          hits: counts[1],
          damages: counts[2] as u16,
          id: next_group,
          attack,
          initiative: counts[3] as u8,
          immunity: Attacks(immunities),
          weakness: Attacks(weaknesses),
        });
        next_group += 1;
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }

  Ok(())
}