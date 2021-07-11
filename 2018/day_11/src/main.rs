use std::env;
use std::process;

fn main() {
  let serial: i32 = env::args()
    .nth(1)
    .unwrap_or_else(|| {
      eprintln!("Usage: day_11 SERIAL");
      process::exit(1);
    })
    .parse()
    .unwrap_or_else(|err| {
      eprintln!("Failed to convert SERIAL into a number: {}", err);
      process::exit(1);
    });
  println!("{}", serial);
}
