mod cli;
mod process;

pub use cli::{Base64Subcommand, Opts, Subcommand};
pub use process::process_csv;
