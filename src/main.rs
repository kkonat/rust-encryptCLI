//custom RC4 code
mod rc4;

use clap::{ArgAction, Parser};
use custom_error::custom_error;
use glob::glob;

use std::fs::{self, metadata};
use std::io;
use std::path::PathBuf;
use std::str;
use std::time::Instant;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const ENCR_FILE_PREFIX: &str = "encr-";

custom_error! {EncError
    Io{source: io::Error} =                 "IO error",
    Pattern{source: glob::PatternError} =   "Glob pattern error",
    Glob{source: glob::GlobError} =         "Glob  error",
    NoFilesFound   =                        "No matching files found",
    GlobFail  =                             "Glob failed",
    MalformedBase64String =                 "Malformed base64 key",
    Other{text: String} =                   "Other error {text}",
    NotAFile =                              "Not a file",
    InvaliDKeyLenght =                      "Invalide key length"
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Flag to indicate key is Base64 encoded
    #[clap(short = 'B', long = "Base64", action = ArgAction::SetTrue)]
    base64enc: bool,

    /// Encryption key
    #[clap(value_parser)]
    key: String,

    // Files to process
    #[clap(value_parser)]
    files: String,
}

fn process_file(filepath: PathBuf, key: &[u8]) -> Result<(), EncError> {
    println!("Encoding:\t{} ...", filepath.display());

    let md = metadata(&filepath)?;
    if md.is_file() {
        let mut buffer = fs::read(&filepath)?;

        let mut rc4 = crate::rc4::RC4::new_rc4(key);
        rc4.apply_cipher(&mut buffer);

        let outfile = format!("{}{}", ENCR_FILE_PREFIX, filepath.to_str().unwrap());

        fs::write(outfile, &buffer)?;

        Ok(())
    } else {
        Err(EncError::NotAFile)
    }
}

fn main() -> Result<(), EncError> {
    let key: Vec<u8>;
    let cli = Cli::parse();

    println!("{} utility v. {}", NAME, VERSION);

    if cli.base64enc {
        if let Ok(k) = base64::decode(cli.key) {
            key = k;
        } else {
            return Err(EncError::MalformedBase64String);
        };
    } else {
        key = cli.key.as_bytes().to_vec();
    }

    let mut processed_files = 0;
    let now = Instant::now();

    for entry in glob(&cli.files[..])? {
        // skip directoriec, etc.
        if process_file(entry?, &key).is_ok() {
            processed_files += 1;
        }
    }

    if processed_files == 0 {
        println!("{}", EncError::NoFilesFound);
    } else {
        println!("-----------------------------");
        println!("Processed:\t{} file(s).", processed_files);
        println!("in {} ms", now.elapsed().as_millis());
    }

    Ok(())
}
