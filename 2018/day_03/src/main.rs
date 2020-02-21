use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

//
//       +------------------------------+     +----------+
//       |                 +---------+  |     |          |
//       |                 |         |  |     +----------+
//   +---+-----------------+---------+--+-----------+
//   |   |                 |         |  |           |
//   |   |                 |         |  |           |               +--------+
//   |   |                 |         |  |      +----+---------------+-+      |
//   |   |                 |         |  |      |    |               | |      |
//   +---+-----------------+---------+--+------+----+               | |      |
//       |                 |         |  |      |                    +-+------+
//       +-----------------+---------+--+      |        +---------+   |
//                         |         |         |        |         |   |
//                         |         |         |        |         |   |
//                         +---------+         |        |         |   |
//                                             |        +---------+   |
//                                             |                      |
//                                             +----------------------+
//
//

#[derive(Debug, Copy, Clone)]
struct Vec2(i32, i32);

#[derive(Debug)]
struct Rect {
  id: i32,
  left_top: Vec2,
  right_bot: Vec2, // right-bottom is exclusive; for simple width/height calcs
}

impl Rect {
  fn new(id: i32, left_top: Vec2, wd_ht: Vec2) -> Self {
    Rect {
      id,
      left_top,
      right_bot: Vec2(left_top.0 + wd_ht.0, left_top.1 + wd_ht.1),
    }
  }

  fn width(&self) -> i32 {
    self.right_bot.0 - self.left_top.0
  }

  fn height(&self) -> i32 {
    self.right_bot.1 - self.left_top.1
  }
}

impl FromStr for Rect {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut fields = s.split_ascii_whitespace();
    let id: i32 = fields.next().unwrap()[1..].parse().unwrap();
    fields.next();
    let coords: Vec<i32> = fields
      .next()
      .unwrap()
      .split(',')
      .map(|num| {
        let n = num.len()
          - match num.chars().last() {
            Some(x) => {
              if x == ':' {
                1
              } else {
                0
              }
            }
            None => 0,
          };
        num[0..=n - 1].parse().unwrap()
      })
      .collect();
    let dims: Vec<i32> = fields
      .next()
      .unwrap()
      .split('x')
      .map(|num| num.parse().unwrap())
      .collect();
    Ok(Rect::new(
      id,
      Vec2(coords[0], coords[1]),
      Vec2(dims[0], dims[1]),
    ))
  }
}

fn main() {
  let rects: Vec<Rect> = io::stdin()
    .lock()
    .lines()
    .map(|lo| Rect::from_str(&lo.unwrap()).unwrap())
    .collect();
}
