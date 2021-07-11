use std::error::Error;
use std::io::{self, BufRead};
use std::num::ParseIntError;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

#[derive(Debug, Copy, Clone)]
struct Vec2(i32, i32);

#[derive(Debug, Copy, Clone)]
struct Light(Vec2, Vec2);

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 600;

impl std::str::FromStr for Light {
  type Err = ParseIntError;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let pt_x: i32 = line[10..16].trim_start().parse().unwrap_or_default();
    let pt_y: i32 = line[18..24].trim_start().parse().unwrap_or_default();
    let d_x: i32 = line[36..38].trim_start().parse().unwrap_or_default();
    let d_y: i32 = line[40..42].trim_start().parse().unwrap_or_default();
    Ok(Light(Vec2(pt_x, pt_y), Vec2(d_x, d_y)))
  }
}

struct Xform {
  sx: f32,
  sy: f32,
  tx: f32,
  ty: f32,
}

//       screen
//           O----------------------------------------> X
//           |                                        |
//           |                                        |
//           |                                        |
//           |                                        |
//           |                                        |
//           |                   sky                  |
//           |                    o------------------>| x
//           |                    |                   |
//           |                    |                   |
//           |                    |                   |
//           |                    |                   |
//           |                    |                   |
//           V--------------------v-------------------+
//           Y                    y              right,bottom
//
// Msky->screen = Tscreen->sky
// scale = screen / sky; translate = screen_width/2, screen_height/2

fn compute_xform(lights: &Vec<Light>) -> Xform {
  let right: i32 = lights
    .iter()
    .max_by(|l, r| l.0 .0.cmp(&r.0 .0))
    .unwrap()
    .0
     .0;
  let left: i32 = lights
    .iter()
    .min_by(|l, r| l.0 .0.cmp(&r.0 .0))
    .unwrap()
    .0
     .0;
  let top: i32 = lights
    .iter()
    .min_by(|l, r| l.0 .1.cmp(&r.0 .1))
    .unwrap()
    .0
     .1;
  let bottom: i32 = lights
    .iter()
    .max_by(|l, r| l.0 .1.cmp(&r.0 .1))
    .unwrap()
    .0
     .1;
  let sky_width: f32 = (right - left) as f32;
  let sky_height: f32 = (bottom - top) as f32;

  // Example Mapping in 1D
  //   sky width = 25; screen width = 8
  //   -10                15  -> sky
  //   |-4|-3|-2|-1|0|1|2|3|  -> screen
  //
  //  scale = 8 / 25 = 0.32
  //  translare = (8 - 2) / 2
  //
  // Even number of pixels mean positive side getting one lesser, reduce on both
  // sides as origin is in centre and divide by two for translate.
  //  -10 → trunc(-0.2) = 0
  //  15 → trunc(7.8) = 7

  // keep buffer of 1 pixel on all sides of screen
  Xform {
    sx: (SCREEN_WIDTH as f32 - 2.0) / (sky_width + 1.0),
    sy: (SCREEN_HEIGHT as f32 - 2.0) / (sky_height + 1.0),
    tx: -left as f32 + 1.0,
    ty: -top as f32 + 1.0,
  }
}

fn move_stars(lights: &mut Vec<Light>, speed: i32) {
  for star in lights.iter_mut() {
    star.0 .0 += speed * (star.1 .0);
    star.0 .1 += speed * (star.1 .1);
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut lights: Vec<Light> = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap_or_default().parse().unwrap())
    .collect();

  let mut opts = WindowOptions::default();
  opts.resize = true;
  let mut wnd = Window::new("Stars Align", SCREEN_WIDTH, SCREEN_HEIGHT, opts)
    .unwrap_or_else(|_| panic!("Window created failed!"));
  wnd.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  let mut xform = compute_xform(&lights);
  let mut frame: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
  let mut pause: bool = false;
  let mut elapsed_sec: i32 = 0;
  let speed: i32 = 64;
  while wnd.is_open() && !wnd.is_key_down(Key::Escape) {
    // render
    frame.fill(0);

    // Adaptive scaling of canvas
    // for each star, apply transform, map to buffer and set colour value
    for star in lights.iter() {
      // translate then scale
      let x = ((star.0 .0 as f32 + xform.tx) * xform.sx).trunc() as usize;
      let y = ((star.0 .1 as f32 + xform.ty) * xform.sy).trunc() as usize;
      frame[(y * SCREEN_WIDTH + x)] = 0xFF_FF_FF_FF;
    }

    wnd
      .update_with_buffer(&frame, SCREEN_WIDTH, SCREEN_HEIGHT)
      .unwrap();

    // update
    if wnd.is_key_pressed(Key::Space, KeyRepeat::No) {
      pause = !pause;
    }
    if !pause {
      elapsed_sec += speed;
      move_stars(&mut lights, speed);
      xform = compute_xform(&lights);
    } else {
      if wnd.is_key_pressed(Key::Right, KeyRepeat::No) {
        elapsed_sec += 1;
        move_stars(&mut lights, 1);
        xform = compute_xform(&lights);
        println!("Seconds: {}", elapsed_sec);
      } else if wnd.is_key_pressed(Key::Left, KeyRepeat::No) {
        elapsed_sec -= 1;
        move_stars(&mut lights, -1);
        xform = compute_xform(&lights);
        println!("Seconds: {}", elapsed_sec);
      }
    }
  }

  Ok(())
}
