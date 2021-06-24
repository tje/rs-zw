use std::io::{self, Read};
use zw::decode;

pub fn main() -> io::Result<()> {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf)?;

  println!("{}", decode(&buf));
  Ok(())
}
