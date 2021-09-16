use std::error::Error;
use std::io::{self, Read};

#[derive(Copy, Clone)]
struct Point(i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
  // allocate 15 KiB memory
  let mut expr = String::with_capacity(15_360);
  // drop the trailing linefeed ‘\n’
  let n = io::stdin().read_to_string(&mut expr)? - 1;
  expr.pop();
  let mut fork_points = Vec::with_capacity(128);
  let mut pos = Point(0, 0);
  let (mut min, mut max) =
    (Point(i32::MAX, i32::MAX), Point(-i32::MAX, -i32::MAX));
  expr
    .bytes()
    .skip_while(|&c| c != b'^')
    .skip(1)
    .take_while(|&c| c != b'$')
    .for_each(|c| {
      match c {
        b'N' => pos.1 += 2,
        b'S' => pos.1 -= 2,
        b'W' => pos.0 -= 2,
        b'E' => pos.0 += 2,
        b'(' => fork_points.push(pos),
        b'|' => pos = *fork_points.last().unwrap(),
        b')' => {
          fork_points.pop();
        }
        _ => unreachable!(),
      }
      min.0 = min.0.min(pos.0);
      max.0 = max.0.max(pos.0);
      min.1 = min.1.min(pos.1);
      max.1 = max.1.max(pos.1);
    });
  // expand by 1 on all sides for the walls
  min.0 -= 1;
  min.1 -= 1;
  max.0 += 1;
  max.1 += 1;
  let dim_x = max.0 - min.0 + 1;
  let dim_y = max.1 - min.1 + 1;
  println!("Map size: {} × {}", dim_x, dim_y);
  println!("Pos: ({}, {})", -min.0, max.1);
  Ok(())
}