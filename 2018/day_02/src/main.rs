use std::io::{self, BufRead};

fn main() {
  let box_ids: Vec<String> =
    io::stdin().lock().lines().map(|l| l.unwrap()).collect();

  // part 1
  let count: [u16; 2] = box_ids
    .iter()
    // map string to array of flags for double and triple char repetitions
    .map(|s| -> [bool; 2] {
      let mut occur = [0u8; 26];
      s.chars() // compute per-char occurance count
        .for_each(|c| occur[c as usize - 'a' as usize] += 1);
      occur
        .iter() // fold all occurances to a single flag array
        .fold([false; 2], |f, &n| [f[0] | (n == 2), f[1] | (n == 3)])
    })
    // accumulate repetition count of all strings
    .fold([0; 2], |acc, s| {
      [acc[0] + s[0] as u16, acc[1] + s[1] as u16]
    });
  println!("checksum: {}", count[0] as u16 * count[1] as u16);
}

/// Return the Levenshtein distance between two string slices
/// which is always ≥ 0.  Codomain: [0, max(s1.len(), s2.len())]
fn levenshtein(s1: &str, s2: &str) -> u32 {
  let len_diff = (s1.len() as isize - s2.len() as isize).abs();
  s1.chars()
    .zip(s2.chars()) // add one if the chars are different
    .fold(0, |diff, c| diff + (c.0 != c.1) as u32)
    + len_diff as u32
}
