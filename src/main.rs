use clap::{self, Parser};
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
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read from stdin");
            let mut encoder = Encodee::new(buffer, 5055, String::from("Vigen"));
            let encoded = encoder.encode();
            println!("{}", encoded);
        }
        Commands::Decode => {
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read from stdin");
            buffer = buffer.trim_end().to_string();

            let mut decoder = Decodee::new(buffer, 5055, String::from("Vigen"));
            let decoded = decoder.decode();
            println!("{}", decoded);
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
