use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::process;
use std::str::FromStr;

use minifb::{Key, Window, WindowOptions};

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

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn draw(canvas: &mut Vec<u32>, r: &Rect) {
  let w = r.width();
  let offset = r.left_top.1 * WIDTH as i32 + r.left_top.0;

  // draw top
  let c = vec![0x80ff0000; w as usize];
  let mut o = offset as usize;
  // 3; 5 -> 3, 4, 5, 6, 7
  canvas.splice(o..(o + w as usize), c.iter().cloned());

  //draw bottom
  o = (offset + (r.height() - 1) * WIDTH as i32) as usize;
  canvas.splice(o..(o + w as usize), c.iter().cloned());

  // draw sides
  for i in 1..r.height() {
    let o = (offset + i * WIDTH as i32) as usize;
    canvas[o] = 0x80ff0000;
    canvas[o - 1 + w as usize] = 0x80ff0000;
    // canvas.splice(o..(o + w as usize), c.iter().cloned());
  }
}

fn draw_rects(canvas: &mut Vec<u32>, rects: &Vec<Rect>) {
  for r in rects {
    draw(canvas, r);
  }
}

fn main() {
  let mut rects: Vec<Rect> = io::stdin()
    .lock()
    .lines()
    .map(|lo| Rect::from_str(&lo.unwrap()).unwrap())
    .collect();

  rects.sort_unstable_by(|a, b| b.right_bot.0.cmp(&a.right_bot.0));
  println!("\n{:?}", rects[0]);
  println!("\n{:?}", rects[1]);
  println!("\n{:?}", rects[2]);
  println!("\n{:?}", rects[3]);

  let mut canvas = vec![0u32; WIDTH * HEIGHT];
  draw_rects(&mut canvas, &rects);

  let mut window = Window::new(
    "Areas claimed by Santa's Elves",
    WIDTH,
    HEIGHT,
    WindowOptions {
      title: true,
      resize: false,
      borderless: false,
      scale: minifb::Scale::X1,
      scale_mode: minifb::ScaleMode::Center,
    },
  )
  .unwrap_or_else(|e| {
    println!("error opening window\n{}", e);
    process::exit(1);
  });

  while window.is_open() && !window.is_key_down(Key::Escape) {
    window
      .update_with_buffer(&canvas, WIDTH, HEIGHT)
      .unwrap_or_else(|e| {
        println!("error drawing to window\n{}", e);
        process::exit(1);
      });
  }
}
