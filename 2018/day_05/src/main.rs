use std::io::{self, Read};

fn main() {
  // polymer.len() ≤ input file size, reserve to avoid reallocation and copy
  const FIFTY_KIB: usize = 1024 * 50;
  // as input is pure ASCII, save 3 bytes per char, use a Vec<u8>, not String
  let mut polymer: Vec<u8> = Vec::with_capacity(FIFTY_KIB);
  for c in io::stdin().lock().bytes() {
    if let Ok(cur) = c {
      let cur_ch = char::from(cur);
      // skip newline char at the end
      if cur_ch.is_ascii_alphabetic() {
        let prev = match polymer.last() {
          Some(last) => *last,
          None => 0,
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
    }
  }
  println!("Reduced polymer length: {}", polymer.len());
}
