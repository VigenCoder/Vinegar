use base64::prelude::*;
use rand::{rng, Rng};

const PARAM_RANGE: (i32, i32) = (0, 629);

const ALPHABET: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub struct Encodee {
  plain: Vec<u8>,
  param: (i32, Vec<u8>),
  cipher: Option<Vec<u8>>,
}

impl Encodee {
  pub fn new(plain: String, param: i32, keyword: String) -> Self {
    assert!(PARAM_RANGE.0 <= param && param <= PARAM_RANGE.1);

    Self {
      plain: BASE64_STANDARD_NO_PAD.encode(plain).into_bytes(),
      param: (param + 10, BASE64_STANDARD_NO_PAD.encode(keyword).into_bytes()),
      cipher: None,
    }
  }

  pub fn encode(&mut self) -> &str {
    if self.cipher.is_none() {
      self.cipher = Some(self.plain.clone());
      self.encode_postfix().encode_keyword().encode_caesar();
    }
    str::from_utf8(self.cipher.as_ref().unwrap()).unwrap()
  }

  fn encode_postfix(&mut self) -> &mut Encodee {
    let cipher = self.cipher.as_mut().unwrap();
    let len = (self.param.0 % 10) as usize;
    cipher.reserve(len);
    let mut rng = rng();
    for _ in 0..len {
      cipher.push(ALPHABET[rng.random_range(0..ALPHABET.len())]);
    }
    self
  }

  fn encode_keyword(&mut self) -> &mut Encodee {
    keyword(self.cipher.as_mut().unwrap(), &self.param.1);
    self
  }

  fn encode_caesar(&mut self) -> &mut Encodee {
    self.cipher.as_mut().unwrap().iter_mut().for_each(|c| {
      *c = from_base64((to_base64(*c) + (self.param.0 / 10) as u8) % 64);
    });
    self
  }
}

pub struct Decodee {
  cipher: Vec<u8>,
  param: (i32, Vec<u8>),
  pub plain: Option<Vec<u8>>,
}

impl Decodee {
  pub fn new(cipher: String, param: i32, keyword: String) -> Self {
    assert!(PARAM_RANGE.0 <= param && param <= PARAM_RANGE.1);

    Self {
      cipher: cipher.into_bytes(),
      param: (param + 10, BASE64_STANDARD_NO_PAD.encode(keyword).into_bytes()),
      plain: None,
    }
  }

  pub fn decode(&mut self) -> &str {
    if self.plain.is_none() {
      self.plain = Some(self.cipher.clone());
      self.decode_caesar().decode_keyword().decode_postfix();
      self.plain = Some(BASE64_STANDARD_NO_PAD.decode(self.plain.as_ref().unwrap()).unwrap());
    }
    str::from_utf8(self.plain.as_ref().unwrap()).unwrap()
  }

  fn decode_postfix(&mut self) -> &mut Decodee {
    let plain = self.plain.as_mut().unwrap();
    plain.truncate(plain.len() - (self.param.0 % 10) as usize);
    self
  }

  fn decode_keyword(&mut self) -> &mut Decodee {
    keyword(self.plain.as_mut().unwrap(), &self.param.1);
    self
  }

  fn decode_caesar(&mut self) -> &mut Decodee {
    self.plain.as_mut().unwrap().iter_mut().for_each(|c| {
      *c = from_base64((to_base64(*c) + 64 - (self.param.0 / 10) as u8) % 64);
    });
    self
  }
}

fn keyword(databytes: &mut Vec<u8>, keybytes: &[u8]) {
  if keybytes.is_empty() {
    return;
  }
  databytes.iter_mut().enumerate()
      .for_each(|(idx, databyte)| {
        *databyte = from_base64(to_base64(*databyte) ^ to_base64(keybytes[idx % keybytes.len()]));
      })
}

fn to_base64(c: u8) -> u8 {
  ALPHABET.iter().position(|&x| x == c).unwrap() as u8
}

fn from_base64(idx: u8) -> u8 {
  ALPHABET[idx as usize]
}
