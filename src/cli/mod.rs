mod base64;
mod csv;

use std::path::Path;

use clap::Parser;

pub use self::base64::Base64Subcommand;
pub use self::csv::{CsvOpts, OutPutFormat};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "show Csv or convert Csv to other format")]
    Csv(CsvOpts),

    #[command(subcommand, name = "base64", about = "base64 encode or decode")]
    Base64(Base64Subcommand),
}

fn check_file_exists(filename: &str) -> Result<String, String> {
    if !Path::new(filename).exists() && filename != "-" {
        Err("specified file not exists".into())
    } else {
        Ok(filename.into())
    }
}

mod tests {
    #[test]
    fn test_check_file_exists() {
        assert_eq!(crate::cli::check_file_exists("-"), Ok("-".into()));
        assert_eq!(
            crate::cli::check_file_exists("Cargo.toml"),
            Ok("Cargo.toml".into())
        );
        assert_eq!(
            crate::cli::check_file_exists("*"),
            Err("specified file not exists".into())
        );
    }
}
