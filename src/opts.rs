use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "show Csv or convert Csv to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    ///输入文件
    #[arg(short, long, value_parser = check_extension)]
    pub input: String,

    ///输出文件
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    ///分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    ///分隔符
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn check_extension(filename: &str) -> Result<String, String> {
    if !filename.ends_with(".csv") {
        Err("unsupported file extension".into())
    } else {
        check_file_exists(filename)
    }
}

fn check_file_exists(filename: &str) -> Result<String, String> {
    if !Path::new(filename).exists() {
        Err("specified file not exists".into())
    } else {
        Ok(filename.into())
    }
}
