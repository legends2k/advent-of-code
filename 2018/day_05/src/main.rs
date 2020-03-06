use std::io::{self, Read};

fn main() {
  let mut polymer = String::new();
  for c in io::stdin().lock().bytes() {
    if let Ok(ch) = c {
      let cur = char::from(ch);
      // skip newline char at the end
      if cur.is_ascii_alphabetic() {
        let prev = match polymer.chars().last() {
          Some(last) => last,
          None => '0',
        };
        if (prev != cur)  // avoid ‘aa’ from passing
          && ((prev == cur.to_ascii_lowercase())
            || (prev == cur.to_ascii_uppercase()))
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
