use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();

    match opt.subcmd {
        Subcommand::Csv(csv_opts) => {
            process_csv(&csv_opts)?;
        }
    }
    Ok(())
}
