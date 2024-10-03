use clap::Parser;
use rcli::{process_csv, Base64Subcommand, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();

    match opt.cmd {
        Subcommand::Csv(csv_opts) => {
            process_csv(&csv_opts)?;
        }
        Subcommand::Base64(cmd) => match cmd {
            Base64Subcommand::Encode(opts) => print!("{:?}", opts),
            Base64Subcommand::Decode(opts) => print!("{:?}", opts),
        },
    }
    Ok(())
}
