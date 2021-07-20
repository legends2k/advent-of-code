use std::{env, process};

const REQUIRED_RECIPES: usize = 10;

fn digits(value: u8) -> (u8, Option<u8>) {
  debug_assert!(value < 100);
  match value < 10 {
    true => (value, None),
    false => (value / 10, Some(value % 10)),
  }
}

fn last_n_recipes(previous_recipes: usize) {
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
  }
  println!(
    "Last 10 recipes after {} recipes: {}",
    previous_recipes,
    recipes[previous_recipes..]
      .iter()
      .take(10) // explicitly take 10 as there’d be 11 if last iter added 2
      .map(|&v| v.to_string())
      .collect::<String>()
  );
}

fn recipes_before(recipe_sequence_str: String) {
  let recipe_sequence: Vec<u8> =
    recipe_sequence_str.bytes().map(|ch| ch - b'0').collect();
  let mut recipes = vec![3u8, 7, 1, 0];
  let (mut i, mut j) = (0, 1);
  loop {
    let (n, opt_m) = digits(recipes[i] + recipes[j]);
    recipes.push(n);
    if recipes.ends_with(&recipe_sequence) {
      break;
    }
    if let Some(m) = opt_m {
      recipes.push(m);
      if recipes.ends_with(&recipe_sequence) {
        break;
      }
    }
    i = (i + 1 + recipes[i] as usize) % recipes.len();
    j = (j + 1 + recipes[j] as usize) % recipes.len();
  }
  let n = recipes.len() - recipe_sequence.len();
  println!("Recipies before arriving at {}: {}", recipe_sequence_str, n);
}

fn main() {
  let input = env::args().nth(1).unwrap_or_else(|| {
    eprintln!("Usage: day_14 RECIPE_COUNT");
    process::exit(1);
  });

  let previous_recipes = input.parse().unwrap_or_else(|err| {
    eprintln!("Failed to convert input into a number: {}", err);
    process::exit(1);
  });

  // Part 1: Last ten recipes after N recipes
  last_n_recipes(previous_recipes);

  // Part 2: Recipes tried before arriving at sequence
  recipes_before(input);
}
