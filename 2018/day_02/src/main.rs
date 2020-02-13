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

  // part 2
  // Test all string pairs (nC₂) for Levenshtein distance ≤ 1.
  // An element needn’t be compared with preceding ones as it’d’ve
  // already been done; hence the skip().
  let s = box_ids
    .iter()
    .enumerate()
    .find_map(|(idx, s1)| {
      box_ids
        .iter()
        .skip(idx + 1) // skip to elements after x
        .find_map(|s2| fuzzy_intersection(s1, s2))
    })
    .unwrap();
  println!("common in box IDs: {}", s);
}

/// Returns intersection of equal length string slices if their
/// Levenshtein distance is ≤ 1.
fn fuzzy_intersection(s1: &str, s2: &str) -> Option<String> {
  assert_eq!(s1.len(), s2.len()); // doesn’t work for unequal strings
  let mut intersection = String::with_capacity(s1.len());
  let mut mismatches = 0;
  for (c1, c2) in s1.chars().zip(s2.chars()) {
    if c1 != c2 {
      mismatches += 1;
      if mismatches > 1 {
        return None;
      }
    } else {
      intersection.push(c1);
    }
  }
  Some(intersection)
}
