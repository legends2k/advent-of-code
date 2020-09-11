use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::{self, ErrorKind};

use std::collections::HashMap;

type Data = u32;

#[derive(Debug)]
struct Node {
  data: Data,
  next: usize,
  prev: usize,
}

struct CircularList {
  list: Vec<Option<Node>>,
  // as long as the list is non-empty this will be pointing to a valid node
  first: usize,
}

impl CircularList {
  fn new() -> Self {
    CircularList {
      list: Vec::<Option<Node>>::with_capacity(16),
      first: 0,
    }
  }

  // inserts data between last and first, becoming the new last
  fn append(&mut self, data: Data) -> usize {
    let mut next = 0;
    let mut prev = 0;
    let n = self.list.len();
    let non_empty = (self.first < n) && self.list[self.first].is_some();
    if non_empty {
      let node_first = self.list[self.first].as_mut().unwrap();
      let cur_prev = node_first.prev;
      next = self.first;
      prev = cur_prev;
      node_first.prev = n;
      self.list[cur_prev].as_mut().unwrap().next = n;
    } else {
      debug_assert_eq!(self.first, 0);
    }
    self.list.push(Some(Node {
      data: data,
      next: next,
      prev: prev,
    }));
    n
  }

  // NOTE: ‘append’ can be generalized into ‘insert_after’
  // expects a non-empty list
  fn insert_after(
    &mut self,
    idx: usize,
    data: Data,
  ) -> Result<usize, Box<dyn Error>> {
    if idx >= self.list.len() || self.list[idx].is_none() {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Node index out of bounds",
      )));
    }
    let n = self.list.len();
    let my_next = self.list[idx].as_ref().unwrap().next;
    self.list.push(Some(Node {
      data: data,
      next: my_next,
      prev: idx,
    }));
    self.list[idx].as_mut().unwrap().next = n;
    self.list[my_next].as_mut().unwrap().prev = n;
    Ok(n)
  }

  // returns the index of node next to the deleted, if theres’s one
  fn delete(&mut self, idx: usize) -> Result<Option<usize>, Box<dyn Error>> {
    if idx >= self.list.len() || self.list[idx].is_none() {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Node index out of bounds",
      )));
    }
    // if this is the last standing element wipe out the list
    if idx == self.first
      && self.list[self.first].as_ref().unwrap().next == self.first
    {
      self.list.clear();
      self.first = 0;
      return Ok(None);
    }

    let node_del = self.list[idx].as_ref().unwrap();
    let prev = node_del.prev;
    let next = node_del.next;
    let node_prev: &mut Node = self.list[prev].as_mut().unwrap();
    node_prev.next = next;
    let node_next: &mut Node = self.list[next].as_mut().unwrap();
    node_next.prev = prev;
    // if the deleted node is the first, make its next the first
    if idx == self.first {
      self.first = next;
    }
    self.list[idx] = None;
    Ok(Some(next))
  }

  // returns ±n-th node from given node
  fn nth(
    &self,
    idx: usize,
    n: isize,
  ) -> std::result::Result<usize, Box<dyn Error>> {
    if idx >= self.list.len() || self.list[idx].is_none() {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Node index out of bounds",
      )));
    }
    let mut ptr = idx;
    let advancer: Box<dyn Fn(usize) -> usize> = match n >= 0 {
      true => Box::new(move |i: usize| self.list[i].as_ref().unwrap().next),
      false => Box::new(move |i: usize| self.list[i].as_ref().unwrap().prev),
    };
    for _ in 0..n.abs() {
      ptr = advancer(ptr);
    }
    Ok(ptr)
  }

  fn data(&self, idx: usize) -> Result<Data, Box<dyn Error>> {
    if idx >= self.list.len() || self.list[idx].is_none() {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Node index out of bounds",
      )));
    }
    Ok(self.list[idx].as_ref().unwrap().data)
  }

  fn set_first(&mut self, idx: usize) -> Result<(), Box<dyn Error>> {
    if idx >= self.list.len() || self.list[idx].is_none() {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Node index out of bounds",
      )));
    }
    self.first = idx;
    Ok(())
  }
}

impl Debug for CircularList {
  fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
    for node in self.list.iter() {
      write!(f, "{:?} -> ", node)?
    }
    Ok(())
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut circle = CircularList::new();
  circle.append(0);

  let mut player_score = HashMap::new();

  let mut player = 0;
  const N_PLAYERS: u32 = 426;
  const MARBLE_VALUE_MAX: u32 = 72058;
  let mut marble_value = 1;

  while marble_value <= MARBLE_VALUE_MAX {
    if (marble_value % 23) != 0 {
      let idx =
        circle.insert_after(circle.nth(circle.first, 1)?, marble_value)?;
      circle.set_first(idx)?;
    } else {
      let del = circle.nth(circle.first, -7)?;
      let removed_marble_value = circle.data(del)?;
      let score = player_score.entry(player).or_insert(0);
      *score += marble_value + removed_marble_value;
      let new_current = circle.delete(del)?;
      if new_current.is_some() {
        circle.set_first(new_current.unwrap())?;
      }
    }
    player = (player + 1) % N_PLAYERS;
    marble_value += 1;
  }

  if let Some((player, score)) =
    player_score.iter().max_by_key(|(_, &score)| score)
  {
    println!("Player {}: {}", player + 1, score);
  }

  Ok(())
}
