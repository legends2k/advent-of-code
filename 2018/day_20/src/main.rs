use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
  let mut expr = String::with_capacity(15_360); // 15 KiB
                                                // drop the trailing linefeed ‘\n’
  let n = io::stdin().read_to_string(&mut expr)? - 1;
  expr.pop();
  Ok(())
}
