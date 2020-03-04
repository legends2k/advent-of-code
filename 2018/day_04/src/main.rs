use std::collections::HashMap;
use std::io::{self, BufRead};

type GuardId = u16;

#[derive(Default)]
struct GuardSchedule {
  asleep_dur: u16,               // in minutes
  sleep_interval: Vec<(u8, u8)>, // [a, b)
}

fn main() {
  // parse and order chronologically
  let mut s: Vec<String> =
    io::stdin().lock().lines().map(|l| l.unwrap()).collect();
  // sort entries by month, day, hour and minute; works as input already has
  // lexicographical ordering; ignore year as they’re all the same
  s.sort_unstable_by(|a, b| a[6..17].cmp(&b[6..17]));

  // part 1: most sleepy guard
  let mut guard_schedule: HashMap<GuardId, GuardSchedule> = HashMap::new();
  let mut id = 0u16;
  let mut slept_at = 0u8;
  for i in s {
    match &i[19..24] {
      "Guard" => id = i[26..].splitn(2, ' ').next().unwrap().parse().unwrap(),
      "falls" => slept_at = i[15..17].parse().unwrap(),
      "wakes" => {
        let woke_at: u8 = i[15..17].parse().unwrap();
        let shed = guard_schedule.entry(id).or_default();
        shed.asleep_dur += (woke_at - slept_at) as u16;
        shed.sleep_interval.push((slept_at, woke_at));
      }
      _ => (),
    }
  }
  let most_slept = guard_schedule
    .iter()
    .max_by(|a, b| a.1.asleep_dur.cmp(&b.1.asleep_dur))
    .unwrap();
}
