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

#[derive(Copy, Clone)]
struct Attacks(u8);

impl Attacks {
  fn to(&self, other: Attacks) -> bool {
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
  initiative: i8,
  immunity: Attacks,
  weakness: Attacks,
}

impl Group {
  fn effective_power(&self) -> i32 {
    self.units as i32 * self.damages as i32
  }

  fn is_alive(&self) -> bool {
    self.units > 0
  }

  fn calc_hit(&self, enemy: &Group) -> i32 {
    match (
      self.immunity.to(enemy.attack),
      self.weakness.to(enemy.attack),
    ) {
      (true, false) => 0,
      (false, false) => enemy.effective_power(),
      (false, true) => enemy.effective_power() * 2,
      (true, true) => unreachable!(),
    }
  }
}

#[derive(Default)]
struct Army<'a> {
  groups: Vec<Group>,
  name: &'a str,
}

impl Army<'_> {
  fn sort_for_attack(&self) -> Vec<u16> {
    let mut ids: Vec<u16> = (0..self.groups.len() as u16).collect();
    ids.sort_by_key(|i|
      // descending sort
      (
        match self.groups[*i as usize].is_alive() {
          true => 0,
          false => 1,
        }
        -self.groups[*i as usize].effective_power(),
        -self.groups[*i as usize].initiative,
      ));
    ids
  }

  fn choose_enemy(&self, order: &Vec<u16>, enemy: &Army) -> Vec<Option<u16>> {
    let mut chosen = vec![false; enemy.groups.len()];
    order
      .iter()
      .map(|i| -> Option<u16> {
        if !self.groups[*i as usize].is_alive() {
          return None;
        }
        let mut enemy_ids: Vec<_> = (0..self.groups.len()).collect();
        enemy_ids.sort_by_cached_key(|&j| {
          let damage_by_i = enemy.groups[j].calc_hit(&self.groups[*i as usize]);
          (
            match enemy.groups[j].is_alive() {
              true => 0,
              false => 1,
            },
            -damage_by_i,
            -enemy.groups[j].effective_power(),
            -enemy.groups[j].initiative,
          )
        });
        match enemy_ids
          .iter()
          .filter(|&j| enemy.groups[*j].is_alive() && !chosen[*j])
          .next()
        {
          Some(&c) => {
            chosen[c] = true;
            Some(c as u16)
          }
          None => None,
        }
      })
      .collect()
  }
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
          initiative: counts[3] as i8,
          immunity: Attacks(immunities),
          weakness: Attacks(weaknesses),
        });
        next_group += 1;
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }

  let ids = [armies[0].sort_for_attack(), armies[1].sort_for_attack()];
  let choices = [
    armies[0].choose_enemy(&ids[0], &armies[1]),
    armies[1].choose_enemy(&ids[1], &armies[0]),
  ];

  // collect all alive groups with respective army ID
  let mut attackers: Vec<(u8, u16)> = ids[0]
    .iter()
    .filter(|&i| choices[0][*i as usize].is_some())
    .map(|&i| (0, i))
    .chain(
      ids[1]
        .iter()
        .filter(|&j| choices[1][*j as usize].is_some())
        .map(|&j| (1, j)),
    )
    .collect::<Vec<(u8, u16)>>();

  attackers.sort_by_key(|&(army_id, group_id)| {
    -armies[army_id as usize].groups[group_id as usize].initiative
  });

  for a in &attackers {
    println!(
      "{}'s Group {} --> Group {:?}",
      armies[a.0 as usize].name, a.1, choices[a.0 as usize][a.1 as usize]
    );
  }

  Ok(())
}
