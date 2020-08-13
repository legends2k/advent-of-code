use std::error::Error;
use std::io::{self, BufRead, ErrorKind};

#[derive(Debug)]
struct NodeHeader {
  node_id: u16,
  metadata_count: u16,
  child_count: u16,
}

struct Node {
  metadata: Vec<u16>,
  children: Vec<u16>,
}

impl Node {
  fn new(data_capacity: usize, child_capacity: usize) -> Self {
    Node {
      metadata: Vec::<u16>::with_capacity(data_capacity),
      children: Vec::<u16>::with_capacity(child_capacity),
    }
  }
}

fn read_number<It>(it: &mut It) -> Result<u16, Box<dyn Error>>
where
  It: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
  Ok(match it.next() {
    Some(n) => {
      let digits = n?;
      // use filter as stdin.lock().split(b' ') clubs the line feed char with
      // the last chunk e.g. 12 32\r becomes ['1', '2'], ['3', '2', \r]. Without
      // this issue one could use String::from_utf8()
      let str_num = digits
        .iter()
        .filter(|d| !d.is_ascii_whitespace())
        .map(|&d| d as char)
        .collect::<String>();
      str_num.parse()?
    }
    None => {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Parse failure for child count",
      )))
    }
  })
}

fn read_header<It>(
  mut it: &mut It,
  node_id: u16,
) -> Result<NodeHeader, Box<dyn Error>>
where
  It: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
  let child_count = read_number(&mut it)?;
  let metadata_count = read_number(&mut it)?;
  Ok(NodeHeader {
    node_id: node_id,
    child_count: child_count,
    metadata_count: metadata_count,
  })
}

fn main() -> Result<(), Box<dyn Error>> {
  let stdin = io::stdin();
  let mut iter = stdin.lock().split(b' ').peekable();

  let mut parsing_nodes = Vec::<NodeHeader>::with_capacity(256);
  let mut nodes = Vec::<Node>::with_capacity(256);
  while iter.peek().is_some() {
    let is_header = match parsing_nodes.last() {
      Some(node_header) => node_header.child_count > 0,
      None => true,
    };
    // print!("Header: {}\t", is_header);
    if is_header {
      let id = nodes.len() as u16;
      if let Some(parent) = parsing_nodes.last_mut() {
        nodes[parent.node_id as usize].children.push(id);
      }
      let header = read_header(&mut iter, id)?;
      // println!("{:?}\t", header);
      nodes.push(Node::new(
        header.metadata_count as usize,
        header.child_count as usize,
      ));
      parsing_nodes.push(header);
    } else {
      let metadata = read_number(&mut iter)?;
      let node_header = parsing_nodes.last_mut().unwrap();
      nodes[node_header.node_id as usize].metadata.push(metadata);
      // println!("  metadata: {} for {}\t", metadata, node_header.node_id);
      node_header.metadata_count -= 1;
      if node_header.metadata_count == 0 {
        parsing_nodes.pop();
        if let Some(parent) = parsing_nodes.last_mut() {
          parent.child_count -= 1;
        }
      }
    }
  }

  let metadata_sum = nodes
    .iter()
    .fold(0u32, |acc, n| acc + n.metadata.iter().sum::<u16>() as u32);
  println!("Sum of metadata entries: {}", metadata_sum);
  // let v1 = read_header(&mut iter);
  // let v2 = read_header(&mut iter);
  // println!("{:?}\n{:?}", v1, v2);

  Ok(())
}
