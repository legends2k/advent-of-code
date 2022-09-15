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

#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($( $args:expr ),*) => { print!( $( $args ),* ); }
}
#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
  ($( $args:expr ),*) => {};
}

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;

#[derive(Copy, Clone)]
struct AttackTypes(u8);

impl AttackTypes {
  fn to(&self, other: AttackTypes) -> bool {
    (other.0 & self.0) != 0
  }
}

impl fmt::Debug for AttackTypes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "0b{:b}", self.0)
  }
}

struct Group {
  units: u32,
  hits: u32,
  damages: u32,
  attack: AttackTypes,
  initiative: i8,
  immunity: AttackTypes,
  weakness: AttackTypes,
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

  fn hit(&mut self, points: i32) {
    let org_units = self.units;
    let units_lost = points as u32 / self.hits;
    self.units = self.units.saturating_sub(units_lost);
    dbg_print!("Units lost: {}\n", org_units - self.units);
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
        !self.groups[*i as usize].is_alive(),
        -self.groups[*i as usize].effective_power(),
        -self.groups[*i as usize].initiative,
      ));
    ids
  }

  fn choose_enemy(&self, order: &Vec<u16>, enemy: &Army) -> Vec<Option<u16>> {
    let mut chosen = vec![false; enemy.groups.len()];
    order
      .iter()
      .map(|idx| {
        let i = *idx as usize;
        if !self.groups[i].is_alive() {
          return None;
        }
        let mut enemy_ids: Vec<_> = (0..enemy.groups.len()).collect();
        enemy_ids.sort_by_cached_key(|&j| {
          let damage_by_i = enemy.groups[j].calc_hit(&self.groups[i]);
          (
            !enemy.groups[j].is_alive(),
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

  fn is_alive(&self) -> bool {
    self.groups.iter().any(|g| g.is_alive())
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

struct Attack {
  army: usize,
  group: usize,
  enemy: usize,
}

impl Attack {
  fn enemy_army(&self) -> usize {
    // make a bool and convert to integral as !1u8 = 254
    (self.army == 0) as usize
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
  let mut attack_to_flag: HashMap<&str, u8> = HashMap::new();
  for line in input.into_inner() {
    match line.as_rule() {
      Rule::army_name => {
        armies[next_army as usize].name = line.as_str();
        next_army += 1;
      }
      Rule::group => {
        let mut counts = [0u32; 4];
        let mut idx = 0;
        let mut attack = AttackTypes(0);
        let mut immunities = 0u8;
        let mut weaknesses = 0u8;
        for r in line.into_inner() {
          match r.as_rule() {
            Rule::count => {
              counts[idx] = u32::from_str(r.as_str())?;
              idx += 1;
            }
            Rule::attack => {
              attack = AttackTypes(to_flag(r.as_str(), &mut attack_to_flag)?);
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
          damages: counts[2],
          attack,
          initiative: counts[3] as i8,
          immunity: AttackTypes(immunities),
          weakness: AttackTypes(weaknesses),
        });
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }

  while armies.iter().all(|a| a.is_alive()) {
    let ids = [armies[0].sort_for_attack(), armies[1].sort_for_attack()];
    let choices = [
      armies[0].choose_enemy(&ids[0], &armies[1]),
      armies[1].choose_enemy(&ids[1], &armies[0]),
    ];

    // Excessive debugging; turn on if needed.
    // for (i, _) in armies.iter().enumerate() {
    //   dbg_print!("Army {}\n", i);
    //   for (idx, &j) in ids[i].iter().enumerate() {
    //     dbg_print!(
    //       "  Group {}: {} --> {:?}\n",
    //       j,
    //       armies[i].groups[j as usize].units,
    //       choices[i][idx]
    //     );
    //   }
    // }

    // collect all alive groups with respective army ID
    let mut fight: Vec<Attack> = ids[0]
      .iter()
      .filter(|&i| armies[0].groups[*i as usize].is_alive())
      .enumerate()
      .filter_map(|(idx, &i)| match choices[0][idx] {
        Some(enemy) => Some(Attack {
          army: 0,
          group: i as usize,
          enemy: enemy.into(),
        }),
        None => None,
      })
      .chain(
        ids[1]
          .iter()
          .filter(|&j| armies[1].groups[*j as usize].is_alive())
          .enumerate()
          .filter_map(|(idx, &j)| match choices[1][idx] {
            Some(enemy) => Some(Attack {
              army: 1,
              group: j as usize,
              enemy: enemy.into(),
            }),
            None => None,
          }),
      )
      .collect::<Vec<Attack>>();

    fight.sort_by_key(|a| -armies[a.army].groups[a.group].initiative);

    for attack in &fight {
      dbg_print!(
        "{}'s Group {} --> {}'s Group {};  ",
        armies[attack.army].name,
        attack.group,
        armies[attack.enemy_army()].name,
        attack.enemy
      );
      let defender = &armies[attack.enemy_army()].groups[attack.enemy];
      let attacker = &armies[attack.army].groups[attack.group];
      let damage = defender.calc_hit(attacker);
      let defender_mut = &mut armies[attack.enemy_army()].groups[attack.enemy];
      defender_mut.hit(damage);
    }
    dbg_print!("--------------\n");
  }
  if let Some(victor) = armies.iter().find(|&a| a.is_alive()) {
    println!(
      "Victor units: {}",
      victor.groups.iter().fold(0, |units, g| units + g.units)
    )
  }

  Ok(())
}
