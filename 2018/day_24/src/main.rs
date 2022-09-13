extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "input.pest"]
pub struct InputParser;

fn main() {
  let input = InputParser::parse(
    Rule::file,
    "Immune System:
5711 units each with 6662 hit points (immune to fire; weak to slashing) with an attack that does 9 bludgeoning damage at initiative 14
2108 units each with 8185 hit points (weak to radiation, bludgeoning) with an attack that does 36 slashing damage at initiative 13

Infection:
1492 units each with 47899 hit points (weak to fire, slashing; immune to cold) with an attack that does 56 bludgeoning damage at initiative 15
8714 units each with 7890 hit points with an attack that does 1 cold damage at initiative 3",
  ).expect("Invalid input").next().unwrap();

  let mut current_army: &str;
  let mut current_group: u32 = 0;
  for line in input.into_inner() {
    match line.as_rule() {
      Rule::army => {
        let mut inner_rules = line.into_inner();
        current_army = inner_rules.next().unwrap().as_str();
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
        println!("    {}: {}", attack, counts[2]);
        println!("    Initiative: {}", counts[3]);
        println!("    Immunities: {:?}", immunities);
        println!("    Weaknesses: {:?}", weaknesses);
        current_group += 1;
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }
}
