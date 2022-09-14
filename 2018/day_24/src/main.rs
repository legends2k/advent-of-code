use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;

use std::{
  collections::HashMap,
  error::Error,
  io::{self, Read},
};

struct Attack(u8);

struct Group {
  units: u32,
  hits: u32,
  attacks: u32,
  attack: Attack,
  initiative: u8,
  immunity: u8,
  weakness: u8,
}

impl Group {
  fn effective_power(&self) -> u32 {
    self.units * self.attacks
  }

  fn is_empty(&self) -> bool {
    self.units <= 0
  }
}

fn to_flag<'a>(
  attack: &'a str,
  attack_to_flag: &mut HashMap<&'a str, u8>,
) -> Result<u8, Box<dyn Error>> {
  let n = attack_to_flag.len();
  assert!(n <= 7);
  Ok(*attack_to_flag.entry(attack).or_insert(1u8 << n))
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut input_str = String::new();
  let mut stdin = io::stdin();
  stdin.read_to_string(&mut input_str)?;
  let input = InputParser::parse(Rule::file, &input_str)
    .expect("Invalid input")
    .next()
    .unwrap();

  let mut current_army: &str;
  let mut current_group: u32 = 0;
  let mut attack_to_flag: HashMap<&str, u8> = HashMap::new();
  for line in input.into_inner() {
    match line.as_rule() {
      Rule::army_name => {
        current_army = line.as_str();
        println!("Army: {}", current_army);
        current_group = 0;
      }
      Rule::group => {
        let mut counts = [""; 4];
        let mut idx = 0;
        let mut attack = "";
        let mut immunities = Vec::<&str>::with_capacity(4);
        let mut weaknesses = Vec::<&str>::with_capacity(4);
        for r in line.into_inner() {
          match r.as_rule() {
            Rule::count => {
              counts[idx] = r.as_str();
              idx += 1;
            }
            Rule::attack => {
              attack = r.as_str();
            }
            Rule::traits => {
              for t in r.into_inner() {
                match t.as_rule() {
                  Rule::immunities => {
                    for i in t.into_inner() {
                      immunities.push(i.as_str());
                    }
                  }
                  Rule::weaknesses => {
                    for w in t.into_inner() {
                      weaknesses.push(w.as_str());
                    }
                  }
                  _ => unreachable!(),
                }
              }
            }
            _ => unreachable!(),
          }
        }
        println!("  Group {}", current_group);
        println!("    Units: {}", counts[0]);
        println!("    Hits: {}", counts[1]);
        println!(
          "    {:?}: {}",
          to_flag(attack, &mut attack_to_flag)?,
          counts[2]
        );
        println!("    Initiative: {}", counts[3]);
        print!("    Immunities: [");
        for i in &immunities {
          print!("{}, ", to_flag(i, &mut attack_to_flag)?);
        }
        print!("]\n    Weaknesses: [");
        for w in &weaknesses {
          print!("{}, ", to_flag(w, &mut attack_to_flag)?);
        }
        println!("]");
        current_group += 1;
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }
  println!("{:?}", attack_to_flag);
  Ok(())
}
