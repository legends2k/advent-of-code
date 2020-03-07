use std::io::{self, Read};

fn fold(cur: u8, polymer: &mut Vec<u8>, filter: &[u8]) {
  if !filter.contains(&cur) {
    let prev = match polymer.last() {
      Some(last) => *last,
      None => 0, // NUL
    };
    if (prev != cur)  // avoid ‘aa’ from passing
      && ((char::from(prev) == char::from(cur).to_ascii_lowercase())
      ||  (char::from(prev) == char::from(cur).to_ascii_uppercase()))
    {
      polymer.pop();
    } else {
      polymer.push(cur);
    }
  }
}

fn main() {
  // polymer.len() ≤ input file size, preallocate to avoid reallocation and copy
  const FIFTY_KIB: usize = 1024 * 50;
  const FIRST: u8 = b'A';
  const LAST: u8 = b'Z';
  const LETTER_COUNT: usize = (LAST - FIRST + 1) as usize;
  const CASE_DIFF: u8 = 32;

  // input is pure ASCII so use a Vec<u8>, not String, save 3 bytes per char
  let mut polymer: Vec<u8> = Vec::with_capacity(FIFTY_KIB);
  let mut filtered_polymers: Vec<Vec<u8>> = vec![polymer.clone(); LETTER_COUNT];

  for c in io::stdin().lock().bytes() {
    if let Ok(cur) = c {
      // skip newline char at the end
      if char::from(cur).is_ascii_alphabetic() {
        fold(cur, &mut polymer, &[]); // part 1

        // part 2
        for (idx, letter) in (FIRST..=LAST).enumerate() {
          let filter = [letter, letter + CASE_DIFF];
          fold(cur, &mut filtered_polymers[idx], &filter);
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
