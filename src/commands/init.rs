use crate::model::config;

pub fn run() {
    config::create_config_file().unwrap();
}
