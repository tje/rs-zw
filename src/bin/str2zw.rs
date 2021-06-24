use std::io::{self, Read};
use zw::encode;

pub fn main() -> io::Result<()> {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf)?;

  println!("{}", encode(&buf));
  Ok(())
}
