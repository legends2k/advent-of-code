use std::collections::HashMap;
use std::io::{self, BufRead};

type GuardId = u16;

#[derive(Default)]
struct GuardSchedule {
  asleep_dur: u16,               // in minutes
  sleep_interval: Vec<(u8, u8)>, // [a, b) in minutes
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

  // part 2
  strategy2(&guard_schedule);
}

// mintue and its occurance frequency pair
struct MinFreq(u8, u8);

/// Returns the most slept minute with frequency
fn most_slept_min(shed: &GuardSchedule) -> Option<MinFreq> {
  let mut occurances = [0u8; 60];
  for i in &shed.sleep_interval {
    (i.0..i.1).for_each(|i| occurances[i as usize] += 1);
  }
  // https://stackoverflow.com/a/58103194/183120
  let max_idx = occurances
    .iter()
    .enumerate()
    .max_by_key(|(_idx, &val)| val) // find max by val
    .map(|(idx, _val)| idx); // but obtain idx as result
  if let Some(idx) = max_idx {
    return Some(MinFreq(idx as u8, occurances[idx]));
  }
  None
}

fn strategy1(guard_schedule: &HashMap<GuardId, GuardSchedule>) {
  let most_slept = guard_schedule
    .iter()
    .max_by(|a, b| a.1.asleep_dur.cmp(&b.1.asleep_dur))
    .unwrap();
  let sleepy_min = most_slept_min(&most_slept.1);
  if let Some(MinFreq(min, freq)) = sleepy_min {
    println!(
      "Strategy 1 \
       \n  Guard #{} slept the most ({} mins); min {} being \
       the most asleep ({} out of {} times).\n  Result: {}",
      most_slept.0,
      most_slept.1.asleep_dur,
      min,
      freq,
      most_slept.1.sleep_interval.len(),
      *most_slept.0 * min as u16
    );
  }
}

fn strategy2(guard_schedule: &HashMap<GuardId, GuardSchedule>) {
  // Calculate max element in guard_schedule manually. Would’ve been nicer if
  // guard_schedule.iter().max_by_key() worked but that only gives max element
  // but not the key calculated for sorting; frequency or the max minute.
  let mut max_freq = 0;
  let mut sel_min = 0;
  let mut guard_id = 0;
  for guard in guard_schedule {
    let sleepy_min = most_slept_min(guard.1);
    if let Some(MinFreq(min, freq)) = sleepy_min {
      if freq > max_freq {
        max_freq = freq;
        sel_min = min;
        guard_id = *guard.0;
      }
    }
  }
  println!(
    "Strategy 2 \
     \n  Guard #{} slept at min {} with highest frequency ({} times) \
     \n  Result: {}",
    guard_id,
    sel_min,
    max_freq,
    guard_id * sel_min as u16
  );
}
