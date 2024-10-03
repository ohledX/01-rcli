use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::check_file_exists;

#[derive(Debug, Clone, Copy)]
pub enum OutPutFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    ///输入文件
    #[arg(short, long, value_parser = check_extension)]
    pub input: String,

    ///输出文件
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, default_value = "json",value_parser = parse_output_str)]
    pub format: OutPutFormat,

    ///分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    ///头
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

fn parse_output_str(out_put: &str) -> Result<OutPutFormat, anyhow::Error> {
    out_put.parse::<OutPutFormat>()
}

impl From<OutPutFormat> for &str {
    fn from(out_put: OutPutFormat) -> Self {
        match out_put {
            OutPutFormat::Json => "json",
            OutPutFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutPutFormat {
    type Err = anyhow::Error;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "json" => Ok(OutPutFormat::Json),
            "yaml" => Ok(OutPutFormat::Yaml),
            v => anyhow::bail!("unsupported output format: {}", v),
        }
    }
}

impl fmt::Display for OutPutFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
