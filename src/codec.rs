use base64::prelude::*;
use getrandom::fill;

pub struct Encodee {
  plain: Vec<u8>,
  params: (u8, u8, u8, Vec<u8>),
  cipher: Vec<u8>,
}

impl Encodee {
  pub fn new(plain: &str, param: u16, keyword: &str) -> Result<Self, String> {
    if param > 4535 {
      return Err("Invalid parameters!".to_string());
    }
    let params = map_param(param, keyword);
    Ok(Self {
      plain: BASE64_STANDARD_NO_PAD.encode(plain).into_bytes(),
      params,
      cipher: vec![],
    })
  }

  pub fn encode(&mut self) -> &str {
    self.cipher = self.plain.clone();
    self.encode_postfix().encode_keyword()
      .encode_caesar().encode_reorder();
    str::from_utf8(&self.cipher).unwrap()
  }

  fn encode_postfix(&mut self) -> &mut Encodee {
    let cipher = &mut self.cipher;
    let len = self.params.0 as usize;
    cipher.reserve(len);
    for _ in 0..len {
      let mut buf = [0u8; 1];
      fill(&mut buf).expect("Generation Failure");
      cipher.push(from_base64(buf[0] % 64));
    }
    self
  }

  fn encode_keyword(&mut self) -> &mut Encodee {
    keyword(&mut self.cipher, &self.params.3);
    self
  }

  fn encode_caesar(&mut self) -> &mut Encodee {
    self.cipher.iter_mut().for_each(|c| {
      *c = from_base64((to_base64(*c) + self.params.1) % 64);
    });
    self
  }

  fn encode_reorder(&mut self) -> &mut Encodee {
    let mut grps: Vec<Vec<u8>> = vec![vec![]; self.params.2 as usize];
    self.cipher.iter().enumerate()
      .for_each(|(idx, &c)| {
        grps[idx % self.params.2 as usize].push(c);
      });
    self.cipher = grps.concat();
    self
  }
}

pub struct Decodee {
  cipher: Vec<u8>,
  params: (u8, u8, u8, Vec<u8>),
  pub plain: Vec<u8>,
}

impl Decodee {
  pub fn new(cipher: &str, param: u16, keyword: &str) -> Result<Self, String> {
    if param > 4535 {
      return Err("Invalid parameters!".to_string());
    }
    let params = map_param(param, keyword);
    Ok(Self {
      cipher: cipher.to_string().into_bytes(),
      params,
      plain: vec![],
    })
  }

  pub fn decode(&mut self) -> Result<&str, &str> {
    self.plain = self.cipher.clone();
    self.decode_reorder().decode_caesar()
      .decode_keyword().decode_postfix();
    let res
      = BASE64_STANDARD_NO_PAD.decode(&self.plain);
    match res {
      Ok(plain) => self.plain = plain,
      Err(_) => return Err("Invalid input!")
    }
    let res = str::from_utf8(&self.plain);
    match res {
      Ok(plain) => Ok(plain),
      Err(_) => Err("Invalid input!")
    }
  }

  fn decode_postfix(&mut self) -> &mut Decodee {
    let plain = &mut self.plain;
    plain.truncate(plain.len() - self.params.0 as usize);
    self
  }

  fn decode_keyword(&mut self) -> &mut Decodee {
    keyword(&mut self.plain, &self.params.3);
    self
  }

  fn decode_caesar(&mut self) -> &mut Decodee {
    self.plain.iter_mut().for_each(|c| {
      *c = from_base64((to_base64(*c) + 64 - self.params.1) % 64);
    });
    self
  }

  fn decode_reorder(&mut self) -> &mut Decodee {
    let len = self.plain.len();
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
      .scan(self.plain.as_slice(), |remaining, &cnt| {
        let (chunk, rest) = remaining.split_at(cnt);
        *remaining = rest;
        Some(chunk.to_vec())
      })
      .collect();
    self.plain =
      (0..*cnts.last().unwrap()).map(|i| {
        grps.iter()
          .map(move |grp| grp[i])
      })
        .flatten()
        .chain(grps.iter()
          .take(len % self.params.2 as usize)
          .map(|grp| grp[grp.len() - 1]))
        .collect();
    self
  }
}

fn keyword(databytes: &mut Vec<u8>, keybytes: &[u8]) {
  if keybytes.is_empty() {
    return;
  }
  databytes.iter_mut().enumerate()
    .for_each(|(idx, databyte)| {
      *databyte = from_base64(to_base64(*databyte) ^
        to_base64(keybytes[idx % keybytes.len()]));
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

fn map_param(param: u16, keyword: &str) -> (u8, u8, u8, Vec<u8>) {
  ((param / 8 / 63 + 1) as u8, (param / 8 % 63 + 1) as u8,
   (param % 8 + 2) as u8,
   BASE64_STANDARD_NO_PAD.encode(keyword).into_bytes())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    let example = "你好{He\u{4e16}llo} 🦀界Wo\nrld！";
    println!("Example: \n{:?}", example);
    let mut encoder =
      Encodee::new(example, 2026, "Vigen").unwrap();
    let cipher = encoder.encode();
    println!("Encoded: \n{:?}", cipher);
    let mut decoder =
      Decodee::new(cipher, 2026, "Vigen").unwrap();
    println!("Decoded: \n{:?}", decoder.decode());
  }

  #[test]
  fn test2() {
    let mut decoder =
      Decodee::new("MFrYo1O19FhfeSqRWEkiDGsZ4", 2026, "Vigen").unwrap();
    println!("Decoded: \n{:?}", decoder.decode());
  }
}
