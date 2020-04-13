use std::char::ParseCharError;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Status {
  Uninitialized,
  Waiting,
  Complete,
}

#[derive(Debug)]
struct Task {
  status: Status,
  awaiting: u8,
  // Inversion of control: dependants instead of dependencies
  // https://en.wikipedia.org/wiki/Inversion_of_control
  // https://stackoverflow.com/q/5792966/183120
  dependants: Vec<u8>,
}

impl Default for Task {
  fn default() -> Self {
    Task {
      status: Status::Uninitialized,
      awaiting: 0,
      dependants: Vec::<u8>::with_capacity(5),
    }
  }
}

impl Task {
  fn is_waiting(&self) -> bool {
    self.status == Status::Waiting
  }
}

fn perform(task_id: u8, tasks: &mut [Task]) {
  debug_assert!(tasks[task_id as usize].is_waiting());
  tasks[task_id as usize].status = Status::Complete;
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
    if dependency >= 26 || this_task >= 26 {
      panic!("Input has more than 26 steps!");
    }
    tasks[this_task as usize].status = Status::Waiting;
    tasks[this_task as usize].awaiting += 1;
    // tasks with no dependency don’t get their own line in the input
    // so set its status too
    tasks[dependency as usize].status = Status::Waiting;
    if !tasks[dependency as usize].dependants.contains(&this_task) {
      tasks[dependency as usize].dependants.push(this_task);
    }
  }

  // part 1 -- task sequencer
  let mut task_order = String::with_capacity(26);
  while tasks.iter().any(|t| t.is_waiting()) {
    let mut available_tasks: Vec<u8> = tasks
      .iter()
      .enumerate()
      .filter(|(_id, task)| task.is_waiting() && task.awaiting == 0)
      .map(|(id, _task)| id as u8)
      .collect();
    available_tasks.sort_unstable();
    perform(available_tasks[0], &mut tasks);
    task_order.push(char::from(b'A' + available_tasks[0]));
  }
  println!("Task sequence: {}", task_order);

  Ok(())
}
