use std::{env, process};

const REQUIRED_RECIPES: usize = 10;

fn digits(value: u8) -> (u8, Option<u8>) {
  debug_assert!(value < 100);
  match value < 10 {
    true => (value, None),
    false => (value / 10, Some(value % 10)),
  }
}

fn main() {
  let previous_recipes: usize = env::args()
    .nth(1)
    .unwrap_or_else(|| {
      eprintln!("Usage: day_14 RECIPE_COUNT");
      process::exit(1);
    })
    .parse()
    .unwrap_or_else(|err| {
      eprintln!("Failed to convert input into a number: {}", err);
      process::exit(1);
    });

  let total_recipes = previous_recipes + REQUIRED_RECIPES;
  let mut recipes = vec![3u8, 7, 1, 0];
  recipes.reserve(total_recipes);
  let (mut i, mut j) = (0, 1);
  // reduce two since they’re already in the vector; add 1 as per puzzle
  while recipes.len() <= total_recipes {
    let (n, opt_m) = digits(recipes[i] + recipes[j]);
    recipes.push(n);
    if let Some(m) = opt_m {
      recipes.push(m);
    }
    i = (i + 1 + recipes[i] as usize) % recipes.len();
    j = (j + 1 + recipes[j] as usize) % recipes.len();
    // println!(
    //   "{}",
    //   recipes
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, &x)| {
    //       match idx {
    //         _ if { idx == i } => format!("({}) ", x),
    //         _ if { idx == j } => format!("[{}] ", x),
    //         _ => format!("{} ", x),
    //       }
    //     })
    //     .collect::<String>()
    // );
  }

  // Part 1: Last ten recipes after N recipes
  println!(
    "{}",
    recipes[previous_recipes..]
      .iter()
      .take(10)
      .map(|&v| v.to_string())
      .collect::<String>()
  );
}
