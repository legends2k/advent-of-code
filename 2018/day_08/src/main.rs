use std::error::Error;
use std::io::{self, BufRead, ErrorKind};

#[derive(Debug)]
struct NodeRecord {
  children: u16,
  metadata: u16,
}

fn read_number<It>(it: &mut It) -> Result<u16, Box<dyn Error>>
where
  It: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
  Ok(match it.next() {
    Some(n) => String::from_utf8(n?)?.parse()?,
    None => {
      return Err(Box::new(io::Error::new(
        ErrorKind::NotFound,
        "Parse failure for child count",
      )))
    }
  })
}

fn read_header<It>(mut it: &mut It) -> Result<NodeRecord, Box<dyn Error>>
where
  It: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
  let child_count = read_number(&mut it)?;
  let metadata_count = read_number(&mut it)?;
  Ok(NodeRecord {
    children: child_count,
    metadata: metadata_count,
  })
}

fn main() -> Result<(), Box<dyn Error>> {
  let nodes = Vec::<NodeRecord>::with_capacity(256);
  let stack = Vec::<u16>::with_capacity(256);

  let stdin = io::stdin();
  let mut iter = stdin.lock().split(b' ');
  let v1 = read_header(&mut iter);
  let v2 = read_header(&mut iter);
  println!("{:?}\n{:?}", v1, v2);

  Ok(())
}
