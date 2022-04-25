use std::{fs::File, io::Write};

use crate::model;

pub fn init() {
    let path = "endpoints.config.json";
    let mut output: File = File::create(path).unwrap();
    let config = model::config::Config::new(None, None, None)
        .publish()
        .unwrap();
    output.write_all(config.as_bytes()).unwrap();
}
