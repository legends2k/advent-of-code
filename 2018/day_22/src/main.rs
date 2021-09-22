use std::error::Error;
use std::io::stdin;

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RegionType {
  Rocky,
  Wet,
  Narrow,
  Mouth,
  Target,
}

struct Map {
  depth: u16,
  width: usize,
  height: usize,
  map: Vec<RegionType>,
}

impl Map {
  fn new(depth: u16, width: usize, height: usize) -> Self {
    Map {
      depth,
      width,
      height,
      map: vec![RegionType::Rocky; width * height],
    }
  }

  fn to_idx(&self, col: usize, row: usize) -> usize {
    row * self.width + col
  }

  fn fill_types(&mut self) {
    // fill erosion levels for left-top borders since their geologic
    // index is indepdendant
    let mut m = vec![0u64; self.width * self.height];
    let mut col = 0;
    let depth = self.depth as u64;
    m[0..self.width].fill_with(|| {
      col += 1;
      (((col - 1) * 16807) + depth) % 20183
    });
    m[0..]
      .iter_mut()
      .step_by(self.width)
      .enumerate()
      .for_each(|(row, p)| *p = ((row as u64 * 48271) + depth) % 20183);

    for y in 1..self.height {
      for x in 1..self.width {
        let geo_index = m[self.to_idx(x - 1, y)] * m[self.to_idx(x, y - 1)];
        m[self.to_idx(x, y)] = (geo_index + depth) % 20183;
      }
    }
    m[(self.width * self.height) - 1] = depth % 20183;

    let mut idx = 0;
    self.map.fill_with(|| {
      idx += 1;
      match m[idx - 1] % 3 {
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        2 => RegionType::Narrow,
        _ => unreachable!(),
      }
    });
  }

  fn risk_level(&self) -> u32 {
    self
      .map
      .iter()
      .map(|&t| match t {
        RegionType::Rocky => 0,
        RegionType::Wet => 1,
        RegionType::Narrow => 2,
        _ => unreachable!(),
      })
      .sum()
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut buf = String::with_capacity(20);
  stdin().read_line(&mut buf)?;
  let depth = buf
    .trim_end()
    .rsplit_once(' ')
    .ok_or("Invalid input")?
    .1
    .parse::<u16>()?;
  buf.clear();
  stdin().read_line(&mut buf)?;
  let target = buf
    .trim_end()
    .rsplit_once(' ')
    .ok_or(
      "Invalid
  input",
    )?
    .1
    .split_once(',')
    .ok_or("Invalid input")?;
  let target_pos = Point(target.0.parse()?, target.1.parse()?);

  let mut m = Map::new(
    depth,
    (target_pos.0 + 1) as usize,
    (target_pos.1 + 1) as usize,
  );
  m.fill_types();
  // Part 1: print risk level of rectangle from cave mouth to target
  println!("{}", m.risk_level());

  Ok(())
}
