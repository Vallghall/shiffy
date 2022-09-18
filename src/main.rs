use std::fs::File;
use std::io::{Read, Write};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use shiffy::cipher::*;

fn main() {
    let cli: Cli = Cli::parse();
    let cipher_map = CipherMap::new();

    let output = match &cli.command {
        Commands::Cipher {file} => {
           cipher_map
               .get(&cli.cipher)
               .expect("bad cipher name")
               .cipher(&read_file(file))
        }
        Commands::Decipher {file} => {
            cipher_map
                .get(&cli.cipher)
                .expect("bad cipher name")
                .decipher(&read_file(file))
        }
    };

    let mut output_file = match &cli.output {
        Some(filename) => File::create(filename).unwrap(),
        None => File::create("output.txt").unwrap()
    };

    output_file.write_all(output.as_bytes()).unwrap();
}

fn read_file(pb: &PathBuf) -> String {
    let mut data = String::new();
    File::open(pb)
        .expect("file does not exist")
        .read_to_string(&mut data)
        .unwrap();
    data
}

#[derive(Debug, Parser)]
#[clap(name = "shiffy")]
#[clap(about = "Ciphering app developed for studying Rust concepts")]
#[clap(author, version, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// Cipher used for ciphering
    #[clap(short, long, value_parser)]
    cipher: String,
    /// Name for the output file
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Ciphers the content within the file specified
    #[clap(arg_required_else_help = true)]
    Cipher {
        /// File to be ciphered
        #[clap(value_parser)]
        file: PathBuf
    },
    /// Deciphers the content within the file specified
    #[clap(arg_required_else_help = true)]
    Decipher {
        /// File to be deciphered
        #[clap(value_parser)]
        file: PathBuf
    }
}
