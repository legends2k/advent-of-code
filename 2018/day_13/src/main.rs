use std::{
  cmp::Ordering,
  fmt::{self, Display, Formatter},
  io::{self, BufRead},
  ops::Add,
};

#[derive(Copy, Clone, Debug)]
enum Turn {
  Left,
  Straight,
  Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point(i32, i32);

// Implement ordering as we want to sort by Y then X; deriving flips it as it
// does lexicographical sorting based on element declaration order
impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.1 != other.1 {
      true => self.1.cmp(&other.1),
      false => self.0.cmp(&other.0),
    }
  }
}

// Implement manually as derive(PartialOrd) breaks consistency as admonitioned
// under PartialOrd (comment, derive and check p1 < p2’s result to verify):
//   Implementations of PartialEq, PartialOrd, and Ord must agree with each
//   other. It’s easy to accidentally make them disagree by deriving some of the
//   traits and manually implementing others.
impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0, self.1 + other.1)
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Cart {
  pos: Point,
  velocity: Point,
  last_turn: Turn,
}

impl Cart {
  fn new(pos: Point, symbol: u8) -> Self {
    let velocity = match symbol {
      b'>' => Point(1, 0),
      b'<' => Point(-1, 0),
      b'v' => Point(0, 1),
      b'^' => Point(0, -1),
      _ => panic!("Unexpected card symbol"),
    };
    Cart {
      pos,
      velocity,
      last_turn: Turn::Right,
    }
  }
}

// Custom ordering for Carts based on |pos|
// https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/
impl Ord for Cart {
  fn cmp(&self, other: &Self) -> Ordering {
    self.pos.cmp(&other.pos)
  }
}

impl PartialOrd for Cart {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Cart {
  fn eq(&self, other: &Self) -> bool {
    self.pos == other.pos
  }
}

impl Eq for Cart {}

struct Map {
  plot: Vec<u8>,
  width: usize,
}

impl Map {
  fn char_at(&self, pt: Point) -> u8 {
    let idx = pt.1 as usize * self.width + pt.0 as usize;
    self.plot[idx]
  }
}

fn update(map: &Map, carts: &mut [Cart]) -> Result<(), Point> {
  let n = carts.len();
  for i in 0..n {
    let new_pos: Point = carts[i].pos + carts[i].velocity;
    // Check for collision from next cart till previous cart circularly; as we
    // simulate top-bottom-left-right, following carts are probable candidates;
    // however last cart’s probably is first, so can’t skip any, cycle through.
    // https://stackoverflow.com/a/59413981/183120
    if carts
      .iter()
      .cycle()
      .skip(i + 1) // start from next cart
      .take(n - 1) // skip self
      .any(|other| other.pos == new_pos)
    {
      return Err(new_pos);
    }
    let c = &mut carts[i];
    c.pos = new_pos;
    let ch = map.char_at(new_pos);
    c.velocity = match (ch, c.velocity, c.last_turn) {
      (b'/', Point(0, -1), _) | (b'\\', Point(0, 1), _) => Point(1, 0),
      (b'/', Point(0, 1), _) | (b'\\', Point(0, -1), _) => Point(-1, 0),
      (b'/', Point(1, 0), _) | (b'\\', Point(-1, 0), _) => Point(0, -1),
      (b'/', Point(-1, 0), _) | (b'\\', Point(1, 0), _) => Point(0, 1),
      (b'+', Point(1, 0), Turn::Right) => Point(0, -1),
      (b'+', Point(-1, 0), Turn::Right) => Point(0, 1),
      (b'+', Point(0, 1), Turn::Right) => Point(1, 0),
      (b'+', Point(0, -1), Turn::Right) => Point(-1, 0),
      (b'+', Point(1, 0), Turn::Straight) => Point(0, 1),
      (b'+', Point(-1, 0), Turn::Straight) => Point(0, -1),
      (b'+', Point(0, 1), Turn::Straight) => Point(-1, 0),
      (b'+', Point(0, -1), Turn::Straight) => Point(1, 0),
      _ => c.velocity,
    };
    c.last_turn = match (ch, c.last_turn) {
      (b'+', Turn::Right) => Turn::Left,
      (b'+', Turn::Left) => Turn::Straight,
      (b'+', Turn::Straight) => Turn::Right,
      _ => c.last_turn,
    };
  }
  Ok(())
}

const CART_SYMBOLS: &[u8; 4] = b"<>^v";

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // fish carts out of map
  let mut plot = Vec::<u8>::new();
  plot.reserve(25600); // my input is 150x150 bytes
  let mut carts = Vec::<Cart>::new();
  let mut height = 0;
  for (line_idx, l) in io::stdin().lock().lines().enumerate() {
    let mut line: Vec<u8> = l?.into_bytes();
    for ch_idx in 0..line.len() {
      let ch = line[ch_idx];
      if CART_SYMBOLS.contains(&ch) {
        line[ch_idx] = match ch {
          b'>' | b'<' => b'-',
          _ => b'|',
        };
        carts.push(Cart::new(Point(ch_idx as i32, line_idx as i32), ch));
      }
    }
    plot.append(&mut line);
    height += 1;
  }
  let width = plot.len() / height;
  let map = Map { plot, width };

  let mut ticks = 0;
  let collision_pt;
  loop {
    match update(&map, &mut carts) {
      Ok(_) => carts.sort(),
      Err(pt) => {
        collision_pt = pt;
        break;
      }
    }
    ticks += 1;
  }
  println!("Collision by tick {} @ {}", ticks, collision_pt);

  Ok(())
}
