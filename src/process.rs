use std::fs;

use csv::Reader;
use serde_json::Value;

use crate::{opts::CsvOpts, OutPutFormat};

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<()> {
    let file_path = csv_opts.input.clone();
    let mut file = Reader::from_path(file_path)?;

    let headers = file.headers()?.clone();
    let ret = file
        .records()
        .map(|record| -> serde_json::Value {
            let rd = record.unwrap();
            headers.iter().zip(rd.iter()).collect::<Value>()
        })
        .collect::<Value>();

    let out_put = if let Some(output) = &csv_opts.output {
        output.clone()
    } else {
        format!("output.{}", csv_opts.format)
    };

    let content = match csv_opts.format {
        OutPutFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutPutFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(out_put, content)?;
    Ok(())
}
