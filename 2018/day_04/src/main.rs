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

  // parse and digest guard schedules
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

  // part 1
  strategy1(&guard_schedule);
}

fn strategy1(guard_schedule: &HashMap<GuardId, GuardSchedule>) {
  let most_slept = guard_schedule
    .iter()
    .max_by(|a, b| a.1.asleep_dur.cmp(&b.1.asleep_dur))
    .unwrap();
  let mut occurances = [0u8; 60];
  for i in &most_slept.1.sleep_interval {
    (i.0..i.1).for_each(|i| occurances[i as usize] += 1);
  }
  // https://stackoverflow.com/a/58103194/183120
  let max_idx = occurances
    .iter()
    .enumerate()
    .max_by_key(|(_idx, &val)| val) // find max by val
    .map(|(idx, _val)| idx); // but obtain idx as result
  if let Some(idx) = max_idx {
    println!(
      "Strategy 1 \
       \n  Guard #{} slept the most ({} mins); min {} being \
       the most asleep ({} out of {} times).\n  Result: {}",
      most_slept.0,
      most_slept.1.asleep_dur,
      idx,
      occurances[idx],
      most_slept.1.sleep_interval.len(),
      *most_slept.0 * idx as u16
    );
  }
}
