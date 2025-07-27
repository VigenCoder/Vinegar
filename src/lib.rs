use wasm_bindgen::prelude::*;
mod codec;

use crate::codec::{Decodee, Encodee};

#[wasm_bindgen]
pub fn encode(plain: &str) -> String {
  let mut encoder = Encodee::new(plain.to_string(), 5055, "Vigen".to_string()).unwrap();
  encoder.encode().to_string()
}

#[wasm_bindgen]
pub fn decode(cipher: &str) -> String {
  let mut decoder = Decodee::new(cipher.to_string(), 5055, "Vigen".to_string()).unwrap();
  match decoder.decode() {
    Ok(cipher) => cipher.to_string(),
    Err(_) => "Invalid input!".to_string()
  }
}