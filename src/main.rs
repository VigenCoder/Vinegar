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
                std::io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
            }

            buffer = buffer.trim_end().to_string();
            let mut encoder = Encodee::new(buffer, 5055, String::from("Vigen"));
            let encoded = encoder.encode();

            if let Some(path) = &cli.output {
                let mut file = std::fs::File::create(path).expect("Failed to create output file");
                file.write_all(encoded.as_bytes())
                    .expect("Failed to write to output file");
            } else {
                std::io::stdout()
                    .write_all(encoded.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
        Commands::Decode => {
            if let Some(path) = &cli.input {
                let mut file = std::fs::File::open(path).expect("Failed to open input file");
                file.read_to_string(&mut buffer)
                    .expect("Failed to read from input file");
            } else {
                std::io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
            }

            buffer = buffer.trim_end().to_string();
            let mut decoder = Decodee::new(buffer, 5055, String::from("Vigen"));
            let decoded = decoder.decode();

            if let Some(path) = &cli.output {
                let mut file = std::fs::File::create(path).expect("Failed to create output file");
                file.write_all(decoded.as_bytes())
                    .expect("Failed to write to output file");
            } else {
                std::io::stdout()
                    .write_all(decoded.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
        Commands::Example => {
            let example = "你好{He\u{4e16}llo} 🦀界Wo\nrld！";
            println!("Example: \n{:?}", example);
            let mut encoder = Encodee::new(example.to_string(), 5055, "Vigen".to_string());
            println!("Encoded: \n{:?}", encoder.encode());
            let mut decoder = Decodee::new(encoder.encode().to_string(), 5055, "Vigen".to_string());
            println!("Decoded: \n{:?}", decoder.decode());
        }
    }
}
