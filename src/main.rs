mod codec;

use crate::codec::{Decodee, Encodee};

fn main() {
  let text = "你好{He\u{4e16}llo} 🦀界Wo\nrld！";
  println!("{}", text);
  let mut encoder = Encodee::new(text.to_string(), 5, "Vigen".to_string());
  println!("{}", encoder.encode());
  let mut decoder = Decodee::new(encoder.encode().to_string(), 5, "Vigen".to_string());
  println!("{}", decoder.decode());
}