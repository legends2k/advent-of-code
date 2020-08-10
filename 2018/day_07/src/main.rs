use std::char::ParseCharError;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
  Uninitialized,
  Waiting,
  Running,
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

impl Clone for Task {
  fn clone(&self) -> Self {
    Task {
      status: self.status,
      awaiting: self.awaiting,
      dependants: self.dependants.clone(),
    }
  }
}

impl Task {
  fn is_waiting(&self) -> bool {
    self.status == Status::Waiting
  }

  fn is_running(&self) -> bool {
    self.status == Status::Running
  }
}

fn perform(task_id: u8, tasks: &mut [Task]) {
  debug_assert!(tasks[task_id as usize].is_running());
  tasks[task_id as usize].status = Status::Complete;
  for &d in &tasks[task_id as usize].dependants {
    tasks[d as usize].awaiting -= 1;
  }
}

#[derive(Default)]
struct ThreadContext {
  task_id: Option<u8>,
  eta: u16,
}

const TOTAL_THREADS: usize = 5;
#[derive(Default)]
struct Processor {
  threads: [ThreadContext; TOTAL_THREADS],
}

// Here we hold a concrete iterator to a struct with known
// parameter; to hold ae genetric/polymorphic iterator see
// https://stackoverflow.com/q/47838596/183120
struct FreeThreadIterator<'a> {
  mut_iter: std::slice::IterMut<'a, ThreadContext>,
}

impl Processor {
  fn free_thread_iter(&mut self) -> FreeThreadIterator {
    FreeThreadIterator {
      mut_iter: self.threads.iter_mut(),
    }
  }

  fn num_free_threads(&self) -> usize {
    // threads with tasks having Status::Waiting are working
    // threads with tasks having Status::Complete are awaiting submission
    self.threads.iter().fold(0, |count, t| {
      count + if t.task_id.is_some() { 0 } else { 1 }
    })
  }
}

impl<'a> Iterator for FreeThreadIterator<'a> {
  type Item = &'a mut ThreadContext;

  fn next(&mut self) -> Option<Self::Item> {
    self.mut_iter.find(|t| t.task_id.is_none())
  }
}

// Alternative approach of implementing FreeThreaditerator; involves unsafe code
//
// struct FreeThreadIterator<'a> {
//   threads: &'a mut [ThreadContext; TOTAL_THREADS],
// }
//
// impl<'a> Iterator for FreeThreadIterator<'a> {
//   type Item = &'a mut ThreadContext;
//
//   fn next(&mut self) -> Option<Self::Item> {
//     match self
//       .threads
//       .iter_mut()
//       .find(|t| t.task_id.is_none())
//     {
//       None => None,
//       // Unsafe code needed to override compiler’s judgement
//       // https://stackoverflow.com/a/62363335/183120
//       // https://stackoverflow.com/q/25730586/183120
//       // https://doc.rust-lang.org/nomicon/borrow-splitting.html#splitting-borrows
//       Some(mut thread) => Some(unsafe {
//         std::mem::transmute::<&mut ThreadContext,
//                               &'a mut ThreadContext>(&mut thread)
//       }),
//     }
//   }
// }

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
  {
    let mut tasks = tasks.clone(); // copy before changing the original
    let mut task_order = String::with_capacity(26);
    while tasks.iter().any(|t| t.is_waiting()) {
      let mut available_tasks: Vec<u8> = tasks
        .iter()
        .enumerate()
        .filter(|(_id, task)| task.is_waiting() && task.awaiting == 0)
        .map(|(id, _task)| id as u8)
        .collect();
      available_tasks.sort_unstable();
      tasks[available_tasks[0] as usize].status = Status::Running;
      perform(available_tasks[0], &mut tasks);
      task_order.push(char::from(b'A' + available_tasks[0]));
    }
    println!("Task sequence: {}", task_order);
  }

  // part 2 -- task runner/simulator
  // Two critical points: task start and complete
  //   Set n_available to 5
  //   Set clock to 0
  //    -- GIVE --
  //   Fetch next available tasks in alphabetic order; for each task
  //     Give, if n_available > 0; mark task status and end time
  //     n_available -= 1
  //    -- RECEIVE --
  //   From busy threads pick one with shortest end time
  //     Set clock to task end time
  //     Mark newly available tasks
  //     n_available += 1
  //   End when task list empty
  let mut cpu = Processor::default();
  let mut time = 0;

  while tasks.iter().any(|t| t.is_waiting()) {
    // give work
    let mut free_threads = cpu.num_free_threads();
    if free_threads > 0 {
      let mut available_tasks: Vec<u8> = tasks
        .iter()
        .enumerate()
        .filter(|(_id, task)| task.is_waiting() && task.awaiting == 0)
        .map(|(id, _task)| id as u8)
        .collect();
      // https://stackoverflow.com/a/60916195/183120
      available_tasks.sort_unstable();
      available_tasks.reverse(); // facilitates poping tasks

      let mut it = cpu.free_thread_iter();

      while free_threads > 0 && !available_tasks.is_empty() {
        let task_id = available_tasks.pop().unwrap();
        let thread = it.next().unwrap();
        thread.task_id = Some(task_id);
        // map A to 61 and store end time upfront; useful in selecting
        // the right thread by ordering w.r.t task end time
        thread.eta = time + task_id as u16 + 61;
        tasks[task_id as usize].status = Status::Running;
        free_threads -= 1;
      }
    }

    // receive result
    if let Some(ready_thread) = cpu
      .threads
      .iter_mut()
      .filter(|t| t.task_id.is_some())
      .min_by_key(|t| t.eta)
    {
      let task_id = ready_thread.task_id.unwrap();
      time = ready_thread.eta;
      perform(task_id, &mut tasks);
      ready_thread.task_id = None;
    }
  }
  println!(
    "Completion of all steps by {} workers: {}",
    TOTAL_THREADS, time
  );

  Ok(())
}
