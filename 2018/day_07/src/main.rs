use std::char::ParseCharError;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Task {
  is_complete: bool,
  awaiting: u8,
  // Inversion of control: dependants instead of dependencies
  // https://en.wikipedia.org/wiki/Inversion_of_control
  // https://stackoverflow.com/q/5792966/183120
  dependants: Vec<u8>,
}

impl Default for Task {
  fn default() -> Self {
    Task {
      is_complete: false,
      awaiting: 0,
      dependants: Vec::<u8>::with_capacity(10),
    }
  }
}

fn perform(task_id: u8, tasks: &mut [Task]) {
  tasks[task_id as usize].is_complete = true;
  for &d in &tasks[task_id as usize].dependants {
    tasks[d as usize].awaiting -= 1;
  }
}

fn main() -> Result<(), ParseCharError> {
  // Array of T initialization, when T ≠ Copy
  // https://www.joshmcguigan.com/blog/array-initialization-rust/
  let mut tasks: [Task; 26] = Default::default();

  // parse input and form dependency graph
  const INSTRUCTION_LENGTH: usize = 48;
  for l in io::stdin().lock().lines() {
    let line = l.unwrap();
    if line.len() != INSTRUCTION_LENGTH {
      panic!(
        "Incorrect instruction length; expected {}",
        INSTRUCTION_LENGTH
      );
    }
    let dependency = line[5..6].parse::<char>()? as u8 - b'A';
    let this_task = line[36..37].parse::<char>()? as u8 - b'A';
    tasks[this_task as usize].awaiting += 1;
    if !tasks[dependency as usize].dependants.contains(&this_task) {
      tasks[dependency as usize].dependants.push(this_task);
    }
  }

  let mut task_order = String::with_capacity(26);
  while let Some(_) = tasks.iter().find(|t| !t.is_complete) {
    let mut available_tasks: Vec<u8> = tasks
      .iter()
      .enumerate()
      .filter(|(_id, task)| !task.is_complete && task.awaiting == 0)
      .map(|(id, _task)| id as u8)
      .collect();
    available_tasks.sort_unstable();
    perform(available_tasks[0], &mut tasks);
    task_order.push(char::from(b'A' + available_tasks[0]));
  }
  println!("{}", task_order);

  Ok(())
}
