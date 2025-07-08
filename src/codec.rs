// use std::collections::HashMap;
use base64::prelude::*;
use rand::{rng, Rng};

const PARAM_RANGE: (i32, i32) = (0, 629);

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
      cipher.push(from_base64(rng.random_range(0..64)));
    }
    self
  }

  fn encode_keyword(&mut self) -> &mut Encodee {
    keyword(self.cipher.as_mut().unwrap(), &self.param.1);
    self
  }
  // 另一种基于关键字的算法
  // fn encode_keyword(&mut self) -> &mut Encodee {
  //   let mut map: HashMap<u8, u8> = HashMap::new();
  //   map.reserve(64);
  //   self.param.1.iter()
  //       .for_each(|&c| {
  //         if !map.contains_key(&to_base64(c)) {
  //           map.insert(to_base64(c), map.len() as u8);
  //         }
  //       });
  //   let mut k = 0;
  //   while k < 64 {
  //     if !map.contains_key(&k) {
  //       map.insert(k, map.len() as u8);
  //     }
  //     k += 1;
  //   }
  //   self.cipher.as_mut().unwrap().iter_mut()
  //       .for_each(|c| {
  //         *c = from_base64(*map.get(&to_base64(*c)).unwrap())
  //       });
  //   self
  // }

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
  // 另一种基于关键字的算法
  // fn decode_keyword(&mut self) -> &mut Decodee {
  //   let mut map: HashMap<u8, u8> = HashMap::new();
  //   map.reserve(64);
  //   self.param.1.iter()
  //       .for_each(|&c| {
  //         if !map.contains_key(&to_base64(c)) {
  //           map.insert(to_base64(c), map.len() as u8);
  //         }
  //       });
  //   let mut k = 0;
  //   while k < 64 {
  //     if !map.contains_key(&k) {
  //       map.insert(k, map.len() as u8);
  //     }
  //     k += 1;
  //   }
  //   let reversed_map: HashMap<u8, u8> = map.into_iter().map(|(k, v)| (v, k)).collect();
  //   self.plain.as_mut().unwrap().iter_mut()
  //       .for_each(|c| {
  //         *c = from_base64(*reversed_map.get(&to_base64(*c)).unwrap())
  //       });
  //   self
  // }

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
  if 65 <= c && c <= 90 {
    c - 65
  } else if 97 <= c && c <= 122 {
    c - 71
  } else if 48 <= c && c <= 57 {
    c + 4
  } else if c == 43 {
    62
  } else if c == 47 {
    63
  } else {
    64
  }
}

fn from_base64(idx: u8) -> u8 {
  if idx <= 25 {
    idx + 65
  } else if 26 <= idx && idx <= 51 {
    idx + 71
  } else if 52 <= idx && idx <= 61 {
    idx - 4
  } else if idx == 62 {
    43
  } else if idx == 63 {
    47
  } else {
    0
  }
}
