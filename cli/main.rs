mod reporter;

use std::path::Path;

use anyhow::Result;
use clap::{arg, Parser, Subcommand};
use rayon::prelude::*;
use yafo::pipeline::ProgressReporter;
use yafo::{Cipher, DecryptState, EncryptState, KeyInit, Pipeline};

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
    #[arg(short, long, help = "The mnemonic phrase to derive the key")]
    pub key: String,
    #[arg(short, long, default_value = "false", help = "Run silently")]
    pub silent: bool,
    #[arg(help = "The file or directory to be encrypted or decrypted")]
    pub input: String,
    #[arg(short, long, default_value = "false", help = "Recursive mode")]
    pub recursive: bool,
}

const YAFO_FILE_EXTENSION: &str = ".yafo";

fn run_pipeline<R, C>(
    pipeline: Pipeline<R>,
    path: &Path,
    cipher: C,
    forward: bool,
    silent: bool,
) -> Result<()>
where
    R: ProgressReporter,
    C: Cipher,
{
    if silent {
        pipeline.process_file(path, cipher)?
    } else {
        pipeline
            .with_progress_reporter(Reporter::new(forward))
            .process_file(path, cipher)?
    }

    Ok(())
}

fn process_file(path: &Path, forward: bool, payload: &Payload) -> Result<()> {
    let pipeline = Pipeline::new().with_buffer();
    let key = payload.key.as_str();
    let silent = payload.silent;

    if forward {
        let encrypt = EncryptState::with_seed_phrase(key);
        run_pipeline(pipeline, path, encrypt, forward, silent)?;

        // Rename the file and add the extension ".yafo" to it.
        let mut new_path = payload.input.clone();
        new_path.push_str(YAFO_FILE_EXTENSION);
        std::fs::rename(&payload.input, &new_path)?;
    } else {
        let decrypt = DecryptState::with_seed_phrase(key);
        run_pipeline(pipeline, path, decrypt, forward, silent)?;

        // Check if the file name has the extension of ".yafo".
        // If it does, remove it. Otherwise, do nothing.
        let file_path = payload.input.clone();
        if let Some(stripped) = file_path.strip_suffix(YAFO_FILE_EXTENSION) {
            std::fs::rename(&file_path, &stripped)?;
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let (forward, payload) = match args.command {
        Commands::Encrypt(payload) => (true, payload),
        Commands::Decrypt(payload) => (false, payload),
    };
    // Check if file exists.
    let path = Path::new(&payload.input);
    let recursive = payload.recursive;

    if !path.exists() {
        eprintln!("file or directory not found: {}", path.display());
        std::process::exit(1);
    }
    if path.is_dir() {
        glob::glob(&format!(
            "{}{}/*",
            glob::Pattern::escape(&payload.input),
            if recursive { "/**" } else { "" }
        ))?
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|entry| match entry {
            Ok(path) => match process_file(path.as_path(), forward, &payload) {
                Err(e) => eprintln!("{}", e),
                _ => (),
            },
            Err(e) => eprintln!("{}", e),
        });
    } else {
        if recursive {
            eprintln!("recursive mode is only supported for a directory");
            std::process::exit(1);
        }
    }

    // process_file(path, forward, &payload)
    Ok(())
}
