mod reporter;

use std::path::Path;

use anyhow::Result;
use clap::{arg, Parser, Subcommand};
use yafo::{DecryptState, EncryptState, KeyInit, Pipeline};

use reporter::Reporter;

#[derive(Debug, Parser)]
#[command(version, about = "Yet Another File Obfuscator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(alias = "e", about = "Encrypt the file")]
    Encrypt(Payload),
    #[command(alias = "d", about = "Decrypt the file")]
    Decrypt(Payload),
}

#[derive(Debug, Clone, Parser)]
pub struct Payload {
    #[arg(short, long)]
    pub key: String,
    #[arg(help = "The file to be encrypted or decrypted")]
    pub input: String,
}

const YAFO_FILE_EXTENSION: &str = ".yafo";

fn main() -> Result<()> {
    let args = Cli::parse();
    let (forward, payload) = match args.command {
        Commands::Encrypt(payload) => (true, payload),
        Commands::Decrypt(payload) => (false, payload),
    };
    // Check if file exists.
    let path = Path::new(&payload.input);
    if !path.exists() {
        eprintln!("File not found: {}", path.display());
        std::process::exit(1);
    }

    let pipeline = Pipeline::new()
        .with_buffer()
        .with_progress_reporter(Reporter::new(forward));
    let key = payload.key.as_str();

    if forward {
        let encrypt = EncryptState::with_seed_phrase(key);
        pipeline.process_file(path, encrypt)?;

        // Rename the file and add the extension ".yafo" to it.
        let mut new_path = payload.input.clone();
        new_path.push_str(YAFO_FILE_EXTENSION);
        std::fs::rename(&payload.input, &new_path)?;
    } else {
        let decrypt = DecryptState::with_seed_phrase(key);
        pipeline.process_file(path, decrypt)?;

        // Check if the file name has the extension of ".yafo".
        // If it does, remove it. Otherwise, do nothing.
        let file_path = payload.input;
        if let Some(stripped) = file_path.strip_suffix(YAFO_FILE_EXTENSION) {
            std::fs::rename(&file_path, &stripped)?;
        }
    };

    Ok(())
}
