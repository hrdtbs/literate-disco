use crate::model::config::*;
use anyhow::Result;
use std::io::prelude::*;
use std::{fs::File, io::Write};

const CONFIG_FILE_NAME: &str = "endpoints.config.json";

pub fn write_config_file(config: Config) -> Result<()> {
    let mut file = File::create(CONFIG_FILE_NAME)?;

    serde_json::to_writer_pretty(&mut file, &config)?;
    Ok(())
}

pub fn create_config_file() -> Result<()> {
    let mut output: File = File::create(CONFIG_FILE_NAME)?;
    let config = Config::new(ConfigOption {
        ..Default::default()
    })
    .publish()?;
    output.write_all(config.as_bytes())?;
    Ok(())
}

pub fn read_config_file() -> Result<Config> {
    let mut file = File::open(CONFIG_FILE_NAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
