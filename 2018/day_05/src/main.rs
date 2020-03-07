use std::io::{self, Read};

fn fold(cur_ch: char, polymer: &mut Vec<u8>, filter: &[u8]) {
  let cur = cur_ch as u8;
  if filter.iter().any(|&f| f == cur) {
    return;
  }
  let prev = match polymer.last() {
    Some(last) => *last,
    None => 0, // NUL
  };
  if (prev != cur)  // avoid ‘aa’ from passing
    && ((char::from(prev) == cur_ch.to_ascii_lowercase())
    || (char::from(prev) == cur_ch.to_ascii_uppercase()))
  {
    polymer.pop();
  } else {
    polymer.push(cur);
  }
}

fn main() {
  // polymer.len() ≤ input file size, reserve to avoid reallocation and copy
  const FIFTY_KIB: usize = 1024 * 50;
  const LETTER_COUNT: usize = 26;
  const FIRST: u8 = 'A' as u8;
  const LAST: u8 = 'Z' as u8;
  const UPPER_LOWER_DIFF: u8 = 32;
  // as input is pure ASCII, save 3 bytes per char, use a Vec<u8>, not String
  let mut polymer: Vec<u8> = Vec::with_capacity(FIFTY_KIB);
  let mut filtered_polymers: Vec<Vec<u8>> = vec![polymer.clone(); LETTER_COUNT];

  for c in io::stdin().lock().bytes() {
    if let Ok(cur) = c {
      let cur_ch = char::from(cur);
      // skip newline char at the end
      if cur_ch.is_ascii_alphabetic() {
        fold(cur_ch, &mut polymer, &[]); // part 1

        // part 2
        for (idx, letter) in (FIRST..=LAST).enumerate() {
          let filtered_letters = [letter, letter + UPPER_LOWER_DIFF];
          fold(cur_ch, &mut filtered_polymers[idx], &filtered_letters);
        }
      }
    }
  }

  // part 1
  println!("Reduced polymer length: {}", polymer.len());

  // part 2
  if let Some(min_idx) = filtered_polymers
    .iter()
    .enumerate()
    .min_by(|(_idx1, v1), (_idx2, v2)| v1.len().cmp(&v2.len()))
    .map(|(idx, _v)| idx)
  {
    println!(
      "Shortest polymer length is with '{}' filtered: {}",
      (min_idx as u8 + FIRST) as char,
      filtered_polymers[min_idx].len()
    );
  }
}
