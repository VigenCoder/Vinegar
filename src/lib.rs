use wasm_bindgen::prelude::*;
mod codec;

use crate::codec::{Decodee, Encodee};

#[wasm_bindgen]
pub fn encode(plain: &str, param: u16, keyword: &str) -> String {
  let mut encoder = Encodee::new(plain, param, keyword).unwrap();
  encoder.encode().to_string()
}

#[wasm_bindgen]
pub fn decode(cipher: &str, param: u16, keyword: &str) -> String {
  let mut decoder = Decodee::new(cipher, param, keyword).unwrap();
  match decoder.decode() {
    Ok(plain) => plain.to_string(),
    Err(_) => "Invalid input!".to_string()
  }
}
