use clap::{self, Parser};
use std::io::{Read, Write};
use std::path::PathBuf;
mod codec;

use crate::codec::{Decodee, Encodee};

#[derive(Parser)]
#[command(
  name = "vinegar",
  version = "0.1.0",
  about = "A simple command line byte stream codec tool"
)]
struct Cli {
  #[command(subcommand)]
  command: Commands,

  #[arg(short, long, help = "input file path")]
  input: Option<PathBuf>,
  #[arg(short, long, help = "output file path")]
  output: Option<PathBuf>,

  #[arg(short, long, help = "enable verbose prompt")]
  verbose: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
  #[command(about = "encode data using the codec")]
  Encode,
  #[command(about = "decode data using the codec")]
  Decode,
  #[command(about = "example encoding and decoding")]
  Example,
}

fn main() {
  let cli = Cli::parse();
  let mut buffer: String = String::new();

  match &cli.command {
    Commands::Encode => {
      if let Some(path) = &cli.input {
        let mut file = std::fs::File::open(path).expect("Failed to open input file");
        file.read_to_string(&mut buffer)
            .expect("Failed to read from input file");
      } else {
        if cli.verbose {
          println!("Please enter the text to encode (Ctrl+D to finish):");
        }
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
      }

      buffer = buffer.trim_end().to_string();

      let encoder = Encodee::new(buffer, 5055, String::from("Vigen"));
      if let Err(error) = encoder {
        eprintln!("Error creating encoder: {}", error);
        return;
      }
      let mut encoder = encoder.unwrap();
      let encoded = encoder.encode();

      if let Some(path) = &cli.output {
        let mut file = std::fs::File::create(path).expect("Failed to create output file");
        file.write_all(encoded.as_bytes())
            .expect("Failed to write to output file");
      } else {
        if cli.verbose {
          println!("Encoded:");
        }
        std::io::stdout()
            .write_all(encoded.as_bytes())
            .expect("Failed to write to stdout");
        if cli.verbose {
          println!();
        }
      }
    }
    Commands::Decode => {
      if let Some(path) = &cli.input {
        let mut file = std::fs::File::open(path).expect("Failed to open input file");
        file.read_to_string(&mut buffer)
            .expect("Failed to read from input file");
      } else {
        if cli.verbose {
          println!("Please enter the text to decode (Ctrl+D to finish):");
        }
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
      }

      buffer = buffer.trim_end().to_string();
      let decoder = Decodee::new(buffer, 5055, String::from("Vigen"));
      if let Err(error) = decoder {
        eprintln!("Error creating decoder: {}", error);
        return;
      }
      let mut decoder = decoder.unwrap();
      let decoded = decoder.decode();
      if let Err(error) = decoded {
        eprintln!("Error decoding: {}", error);
        return;
      }
      let decoded = decoded.unwrap();

      if let Some(path) = &cli.output {
        let mut file = std::fs::File::create(path).expect("Failed to create output file");
        file.write_all(decoded.as_bytes())
            .expect("Failed to write to output file");
      } else {
        if cli.verbose {
          println!("Decoded:");
        }
        std::io::stdout()
            .write_all(decoded.as_bytes())
            .expect("Failed to write to stdout");
        if cli.verbose {
          println!();
        }
      }
    }
    Commands::Example => {
      let example = "你好{He\u{4e16}llo} 🦀界Wo\nrld！";
      println!("Example: \n{:?}", example);
      let mut encoder = Encodee::new(example.to_string(), 5055, "Vigen".to_string()).unwrap();
      let cipher = encoder.encode();
      println!("Encoded: \n{:?}", cipher);
      let mut decoder = Decodee::new(cipher.to_string(), 5055, "Vigen".to_string()).unwrap();
      println!("Decoded: \n{:?}", decoder.decode());
    }
  }
}
