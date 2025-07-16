use base64::prelude::*;
use rand::{rng, Rng};

pub const PARAM_RANGE: ((u8, u8), (u8, u8), (u8, u8)) = ((1, 9), (1, 63), (2, 9));

pub struct Encodee {
  plain: Vec<u8>,
  params: (u8, u8, u8, Vec<u8>),
  cipher: Option<Vec<u8>>,
}

impl Encodee {
  pub fn new(plain: String, param: u16, keyword: String) -> Result<Self, String> {
    let params = ((param / 1000) as u8, (param / 10 % 100) as u8, (param % 10) as u8, BASE64_STANDARD_NO_PAD.encode(keyword).into_bytes());
    match valid_param(&params) {
      true => {
        Ok(Self {
          plain: BASE64_STANDARD_NO_PAD.encode(plain).into_bytes(),
          params,
          cipher: None,
        })
      }
      false => Err("Invalid parameters!".to_string())
    }
  }

  pub fn encode(&mut self) -> &str {
    self.cipher = Some(self.plain.clone());
    self.encode_postfix().encode_keyword().encode_caesar().encode_reorder();
    str::from_utf8(self.cipher.as_ref().unwrap()).unwrap()
  }

  fn encode_postfix(&mut self) -> &mut Encodee {
    let cipher = self.cipher.as_mut().unwrap();
    let len = self.params.0 as usize;
    cipher.reserve(len);
    let mut rng = rng();
    for _ in 0..len {
      cipher.push(from_base64(rng.random_range(0..64)));
    }
    self
  }

  fn encode_keyword(&mut self) -> &mut Encodee {
    keyword(self.cipher.as_mut().unwrap(), &self.params.3);
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
      *c = from_base64((to_base64(*c) + self.params.1) % 64);
    });
    self
  }

  fn encode_reorder(&mut self) -> &mut Encodee {
    let mut grps: Vec<Vec<u8>> = vec![vec![]; self.params.2 as usize];
    self.cipher.as_ref().unwrap().iter().enumerate()
        .for_each(|(idx, &c)| {
          grps[idx % self.params.2 as usize].push(c);
        });
    self.cipher = Some(grps.concat());
    self
  }
}

pub struct Decodee {
  cipher: Vec<u8>,
  params: (u8, u8, u8, Vec<u8>),
  pub plain: Option<Vec<u8>>,
}

impl Decodee {
  pub fn new(cipher: String, param: u16, keyword: String) -> Result<Self, String> {
    let params = ((param / 1000) as u8, (param / 10 % 100) as u8, (param % 10) as u8, BASE64_STANDARD_NO_PAD.encode(keyword).into_bytes());
    match valid_param(&params) {
      true => {
        Ok(Self {
          cipher: cipher.into_bytes(),
          params,
          plain: None,
        })
      }
      false => Err("Invalid parameters!".to_string())
    }
  }

  pub fn decode(&mut self) -> Result<&str, &str> {
    self.plain = Some(self.cipher.clone());
    self.decode_reorder().decode_caesar().decode_keyword().decode_postfix();
    let res = BASE64_STANDARD_NO_PAD.decode(self.plain.as_ref().unwrap());
    match res {
      Ok(_) => self.plain = Some(BASE64_STANDARD_NO_PAD.decode(self.plain.as_ref().unwrap()).unwrap()),
      Err(_) => return Err("Invalid input!")
    }
    Ok(str::from_utf8(self.plain.as_ref().unwrap()).unwrap())
  }

  fn decode_postfix(&mut self) -> &mut Decodee {
    let plain = self.plain.as_mut().unwrap();
    plain.truncate(plain.len() - self.params.0 as usize);
    self
  }

  fn decode_keyword(&mut self) -> &mut Decodee {
    keyword(self.plain.as_mut().unwrap(), &self.params.3);
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
      *c = from_base64((to_base64(*c) + 64 - self.params.1) % 64);
    });
    self
  }

  fn decode_reorder(&mut self) -> &mut Decodee {
    let len = self.plain.as_ref().unwrap().len();
    let cnts: Vec<usize> = (0..self.params.2 as usize)
        .map(|i| {
          if i < (len % self.params.2 as usize) {
            (len / self.params.2 as usize) + 1
          } else {
            len / self.params.2 as usize
          }
        })
        .collect();
    let grps: Vec<_> = cnts.iter()
        .scan(self.plain.as_ref().unwrap().as_slice(), |remaining, &cnt| {
          let (chunk, rest) = remaining.split_at(cnt);
          *remaining = rest;
          Some(chunk.to_vec())
        })
        .collect();
    self.plain = Some(
      (0..*cnts.last().unwrap()).map(|i| {
        grps.iter()
            .map(move |grp| grp[i])
      })
          .flatten()
          .chain(grps.iter()
              .take(len % self.params.2 as usize)
              .map(|grp| grp[grp.len() - 1]))
          .collect()
    );
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

#[test]
fn test_decode() {
  // let mut decoder = Decodee::new("123344".to_string(), 5055, "Vigen".to_string());
  // println!("Decoded: \n{:?}", decoder.decode());
  let example = "你好{He\u{4e16}llo} 🦀界Wo\nrld！";
  println!("Example: \n{:?}", example);
  let mut encoder = Encodee::new(example.to_string(), 5055, "Vigen".to_string()).unwrap();
  println!("Encoded: \n{:?}", encoder.encode());
  let mut decoder = Decodee::new(encoder.encode().to_string(), 5055, "Vigen".to_string()).unwrap();
  println!("Decoded: \n{:?}", decoder.decode());
}

fn valid_param(params: &(u8, u8, u8, Vec<u8>)) -> bool {
  PARAM_RANGE.0.0 <= params.0 && params.0 <= PARAM_RANGE.0.1 && PARAM_RANGE.1.0 <= params.1 && params.1 <= PARAM_RANGE.1.1 && PARAM_RANGE.2.0 <= params.2 && params.2 <= PARAM_RANGE.2.1
}