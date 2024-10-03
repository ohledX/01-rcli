use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode",value_parser = check_file_exists, about = "base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, default_value = "-")]
    pub output: String,
}
