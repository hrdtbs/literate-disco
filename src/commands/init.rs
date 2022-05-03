use std::{fs::File, io::Write};

use crate::model::config::{Config, ConfigOption};

pub fn run() {
    let path = "endpoints.config.json";
    let mut output: File = File::create(path).unwrap();
    let config = Config::new(ConfigOption {
        ..Default::default()
    })
    .publish()
    .unwrap();
    output.write_all(config.as_bytes()).unwrap();
}
